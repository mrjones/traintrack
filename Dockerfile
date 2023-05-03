FROM ubuntu:22.04
MAINTAINER Matt Jones <jonesmr@gmail.com>

RUN apt-get update && apt-get -y install \
  ca-certificates \
  libssl-dev

COPY bin/server /deploy/server
COPY templates /deploy/templates
COPY static /deploy/static
COPY data /deploy/data
COPY webclient/dist/bin/webclient.js /deploy/webclient.js
COPY webclient/dist/bin/webclient.js.gz /deploy/webclient.js.gz
RUN mkdir -p /deploy/log

EXPOSE 3837

ENTRYPOINT [ \
  "/deploy/server", \
  "--port", "3837", \
  "--root-directory", "/deploy", \
  "--webclient-js-file", "/deploy/webclient.js.gz", \
  "--webclient-js-gzipped" \
]
