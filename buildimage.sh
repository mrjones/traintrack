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

push="false"
if [[ $2 == "push" ]]
then
    push="true"
fi

echo "=== Compiling binaries"
TRAINTRACK_VERSION=${tag} cargo build --color=never --release
mkdir -p bin
cp target/release/traintrack bin/server
cp target/release/feedproxy build/feedproxy/feedproxy

echo "=== Compiling JavaScript/WebClient"
pushd .
cd webclient
webpack -p
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

if [[ $push == "true" ]]
then
    echo "=== Pushing to docker hub"
    gcloud docker -- push ${frontendGcrName}:${tag}
    gcloud docker -- push ${feedproxyGcrName}:${tag}

else
    echo "=== Skipping push to docker hub"
fi
