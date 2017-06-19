FROM ubuntu:16.04
MAINTAINER Matt Jones <jonesmr@gmail.com>

RUN apt-get update && apt-get -y install libssl-dev

COPY feedproxy /deploy/feedproxy
RUN mkdir -p /deploy/log

ENTRYPOINT [ \
  "/deploy/feedproxy", \
  "--root-directory", "/deploy" \
]
