#!/bin/bash

rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release

docker build -t $IMAGES .

if $PUSH_IMAGE
then
    docker push $IMAGES
fi
