usage="$0 [tag]"
project="mrjones/traintrack"
imageName="gcr.io/mrjones-gke/traintrack"

set -e

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
cargo build --color=never --release
mkdir -p bin
cp target/release/traintrack bin/server
cp target/release/feedproxy build/feedproxy/feedproxy

echo "=== Compiling JavaScript/WebClient"
pushd .
cd webclient
webpack -p
popd

echo "=== Creating frontend image"
docker build -t $project:$tag .
docker tag $project:$tag ${imageName}:${tag}

echo "=== Creating feedproxy image"
feedproxyLocalName="mrjones/traintrack-feedproxy"
feedproxyGcrTag="gcr.io/mrjones-gke/traintrack-feedproxy"

docker build -t $feedproxyLocalName:$tag build/feedproxy
docker tag $feedproxyLocalName:$tag $feedproxyGcrTag:${tag}

if [[ $push == "true" ]]
then
    echo "=== Pushing to docker hub"
    gcloud docker push ${imageName}:${tag}
    gcloud docker push ${feedproxyGcrTag}:${tag}

else
    echo "=== Skipping push to docker hub"
fi
