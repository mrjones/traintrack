FROM ubuntu:18.04
MAINTAINER Matt Jones <jonesmr@gmail.com>

RUN apt-get update && apt-get -y install \
  ca-certificates \
  libssl1.0.0 \
  libssl-dev

COPY feedproxy /deploy/feedproxy
RUN mkdir -p /deploy/log

ENTRYPOINT [ \
  "/deploy/feedproxy", \
  "--root-directory", "/deploy" \
]
