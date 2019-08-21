#!/bin/bash

cargo build --release

docker build -t $IMAGES .

if $PUSH_IMAGE
then
    docker push $IMAGES
fi
