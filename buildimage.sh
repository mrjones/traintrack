!/bin/bash -eu
#
# Copyright 2018 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

usage="$0 [tag]"

echo "Docker version $(docker --version)"

set -x

if [ -z $1 ]
then
    echo "Please supply a tag for this image"
    echo "Run 'gcloud docker images' to see existing images / tags"
    echo $usage
    exit 1
fi
tag=$1

echo "=== Compiling binaries"
TRAINTRACK_VERSION=${tag} cargo build --color=never --release
mkdir -p bin
cp target/release/traintrack bin/server
cp target/release/feedproxy build/feedproxy/feedproxy

echo "=== Compiling JavaScript/WebClient"
pushd .
cd webclient
TRAINTRACK_VERSION=${tag} webpack --mode=production --config webpack-prod.config.js
popd

echo "=== Creating frontend image"
frontendLocalName="mrjones/traintrack"
frontendGcrName="gcr.io/mrjones-gke/traintrack"
docker build -t $frontendLocalName:$tag .
docker tag $frontendLocalName:$tag ${frontendGcrName}:${tag}

echo "=== Creating feedproxy image"
feedproxyLocalName="mrjones/traintrack-feedproxy"
feedproxyGcrName="gcr.io/mrjones-gke/traintrack-feedproxy"

docker build -t $feedproxyLocalName:$tag build/feedproxy
docker tag $feedproxyLocalName:$tag $feedproxyGcrName:${tag}

echo "=== Pushing to docker hub"
gcloud auth configure-docker
docker push ${frontendGcrName}:${tag}
docker push ${feedproxyGcrName}:${tag}
