t=$(mktemp)
wget --quiet $1 -O $t
protoc --decode StationStatus -I proto proto/webclient_api.proto < ${t}
