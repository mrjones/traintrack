FROM ubuntu:16.04
MAINTAINER Matt Jones <jonesmr@gmail.com>

RUN apt-get update && apt-get -y install libssl-dev

COPY bin/server /deploy/server
COPY templates /deploy/templates
COPY static /deploy/static
COPY data /deploy/data
RUN mkdir -p /deploy/log

EXPOSE 3837

ENTRYPOINT [ \
  "/deploy/server", \
  "--port", "3837", \
  "--root-directory", "/deploy" \
]
