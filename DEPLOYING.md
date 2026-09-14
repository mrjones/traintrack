# Deploying TrainTrack

How to cut a release and push it to production (http://traintrack.nyc).

Releases are infrequent — assume you have forgotten everything. Work through
this top to bottom.

## Overview

Two images are built from this repo, both into `gcr.io/mrjones-gke`:

| Image | Built from | Deployed by |
| --- | --- | --- |
| `traintrack` (frontend) | `Dockerfile` + `bin/server` | `frontend-kube.yaml` |
| `traintrack-feedproxy` | `build/feedproxy/Dockerfile` | `feedproxy-kube.yaml` |

`buildimage.sh` **always builds and pushes both**, but pushing an image does not
deploy it. A change only goes live when the `image:` tag in the corresponding
`*-kube.yaml` is bumped and applied. Most changes are frontend-only, so
normally you only apply `frontend-kube.yaml`.

## Version tags

Format is `YYYY-MM-DD.N`, where `N` starts at `0` and increments if you cut more
than one release in a day — e.g. `2025-01-03.1`, `2026-09-13.0`.

The tag is the Docker image tag *and* the user-visible build label: it is passed
as `TRAINTRACK_VERSION` into both `cargo build` (via `build.rs`) and the webpack
prod config (`BUILD_LABEL`).

List what is already published:

```bash
gcloud container images list-tags gcr.io/mrjones-gke/traintrack --limit=10
```

## Prerequisites

Check all of these *before* building — several fail late and waste a full
release build.

```bash
docker info                      # daemon must be running
cargo --version
gcloud auth list                 # must show an ACTIVE account
kubectl config current-context   # must be gke_mrjones-gke_us-east1_cluster-2
```

**gcloud credentials expire.** `gcloud auth list` showing an account is *not*
enough — the refresh token may still be dead. Confirm with a real API call:

```bash
gcloud container images list-tags gcr.io/mrjones-gke/traintrack --limit=1
```

If that errors with `invalid_grant: Bad Request`, re-authenticate:

```bash
gcloud auth login
```

**Node dependencies** must be installed, or the webpack step fails:

```bash
cd webclient && npm install
```

## Release steps

### 1. Sync and pick a tag

```bash
git pull --ff-only
```

If you have local work in progress, see
[Do not ship uncommitted WIP](#do-not-ship-uncommitted-wip) below.

### 2. Update the About page

`webclient/src/about-page.tsx` has a "Recent changes" list, newest entry at the
bottom. Add a one-line entry dated to match the release tag. If the change came
from an outside contributor, credit them with a link to their GitHub profile.

### 3. Build and push both images

```bash
./buildimage.sh 2026-09-13.0
```

This compiles the Rust binaries in release mode, bundles the webclient JS,
builds both Docker images, tags them for GCR, and pushes them.

### 4. Bump the deployed image tag

Edit the `image:` line in `frontend-kube.yaml`:

```yaml
image: gcr.io/mrjones-gke/traintrack:2026-09-13.0
```

Only touch `feedproxy-kube.yaml` if the feedproxy actually changed — see the
warning below before you do.

### 5. Apply and watch the rollout

```bash
kubectl apply -f frontend-kube.yaml
kubectl rollout status deployment/traintrack
```

`kubectl rollout status` blocks until all replicas are healthy, and is the real
signal that the release is good. The deployment runs 2 replicas with a liveness
probe on `/` at port 3837, so a server that panics on startup shows up as pods
stuck in `CrashLoopBackOff` rather than a clean failure.

### 6. Verify

Confirm the new pods are running with zero restarts — a restart count climbing
from 0 means the server is crashing on startup:

```bash
kubectl get pods -l run=traintrack
kubectl get deployment traintrack -o jsonpath='{.spec.template.spec.containers[0].image}'
```

Then verify the release actually reached users. The About page is
client-rendered, so `curl` on `/` will not show it — check the served JS bundle
instead:

```bash
curl -s --compressed http://traintrack.nyc/webclient.js | grep -c '2026-09-13.0'
```

`BUILD_LABEL` is compiled into the bundle, so finding the tag there proves the
new image is live and not a cached old one.

### Rolling back

`revisionHistoryLimit: 5`, so the previous release is always available:

```bash
kubectl rollout undo deployment/traintrack
```

## Gotchas

### Docker ENTRYPOINT flags must match the Rust flags

This is the easiest way to take the site down, because nothing catches it until
pods are already crashlooping in production.

The server parses flags with `getopts`, which **panics on an unrecognized
flag**. The flags are hardcoded in the `ENTRYPOINT` of `Dockerfile` and
`build/feedproxy/Dockerfile`. Renaming or removing a flag in `src/main.rs` or
`src/feedproxy_main.rs` without updating the matching Dockerfile produces an
image that builds fine and then fails instantly at startup.

Local dev does not catch this: `frontend.sh` and `proxy.sh` pass their own flags
and never exercise the Dockerfile `ENTRYPOINT`.

Before building, diff the two sets:

```bash
grep -n '^\s*"--' Dockerfile build/feedproxy/Dockerfile
grep -n 'opts.optopt' src/main.rs src/feedproxy_main.rs
```

A quick way to prove an image is startable before deploying it — the container
should stay up rather than exit:

```bash
timeout 20 docker run --rm mrjones/traintrack-feedproxy:TAG --mta-api-key=fake
```

Exit code `124` (killed by `timeout`) means it started fine. Exit `101` is a
Rust panic — read the message, it names the offending flag.

**Past instance, now fixed:** commit `a7b13bd` replaced `--root-directory` with
`--log-dir` in `src/feedproxy_main.rs` but left `build/feedproxy/Dockerfile`
passing `--root-directory /deploy`. That went unnoticed for over a year because
the feedproxy image was not rebuilt in that window. Any feedproxy image built
between `a7b13bd` and the fix — including `2026-09-13.0` — panics immediately on
startup and must not be deployed.

### Do not ship uncommitted WIP

`buildimage.sh` builds the working tree, not `HEAD`. Uncommitted changes get
silently baked into the image. Check before building:

```bash
git status -s
```

Stash anything not meant for the release:

```bash
git stash push -m "not in this release" <paths>
```

### There are two GKE clusters

`kubectl config get-contexts` lists both. Production is
`gke_mrjones-gke_us-east1_cluster-2`. The older
`gke_mrjones-gke_us-central1-f_cluster-1` is *not* production — note that
`admin-kube.sh` still refers to `cluster-1`, so do not use it to set your
context. To point at production explicitly:

```bash
gcloud container clusters get-credentials cluster-2 --project mrjones-gke --region us-east1
```

### The webclient build breaks on missing dependencies

`webclient/package.json` has historically drifted out of sync with what the
source actually imports — commit `9afa6b2` ("updated all webclient dependencies")
pruned packages that are still referenced, and some were never listed at all.
Because releases are months apart, this only surfaces mid-build.

Three were restored during the `2026-09-13.0` release and are now pinned in
`package.json`:

| Package | Needed by | Symptom if missing |
| --- | --- | --- |
| `babel-loader` | `webpack.config.js` loader chain | `Can't resolve 'babel-loader'` |
| `es-cookie` | `src/recent-stations.ts` | `Can't resolve 'es-cookie'` |
| `@types/long` | `src/station-view.tsx` (`Long` type) | `TS2304: Cannot find name 'Long'` |

If the webpack step fails on an unresolved module, install the package with
`npm install --save` (or `--save-dev` for types/loaders) so it is captured in
`package.json` and `package-lock.json` rather than existing only in your local
`node_modules`.

### webpack is not on PATH

`buildimage.sh` invokes `./node_modules/.bin/webpack` from inside `webclient/`
rather than a bare `webpack`, because webpack is a local dev dependency and is
not installed globally.

### imagePullPolicy is IfNotPresent

`frontend-kube.yaml` sets `imagePullPolicy: IfNotPresent`, so **never reuse a
tag**. Pushing new content to an existing tag will not reliably roll out — nodes
that already cached that tag keep the old image. Always increment the version.

## Secrets

Secrets already exist in the cluster and are not part of a normal release. They
are referenced by `frontend-kube.yaml` as `traintrack-google-creds`,
`traintrack-firebase-creds`, and the mounted
`mrjones-traintrack-service-account-key`. The commands that originally created
them are recorded in the comments at the top of `frontend-kube.yaml`.

For local development secrets, see the "Secrets Management" section of
`README.md`.
