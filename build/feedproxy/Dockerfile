FROM ubuntu:16.04
MAINTAINER Matt Jones <jonesmr@gmail.com>

COPY feedproxy /deploy/feedproxy
RUN mkdir -p /deploy/log

ENTRYPOINT [ \
  "/deploy/feedproxy", \
  "--root-directory", "/deploy" \
]
