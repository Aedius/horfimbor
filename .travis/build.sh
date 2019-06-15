#!/bin/bash

if .travis/build-condition.sh $TRAVIS_COMMIT_RANGE $PROJECT; then
    echo "$PROJECT is being built"
    cd $TRAVIS_BUILD_DIR/$PROJECT
    cargo build --verbose --all
else
    echo "$PROJECT is NOT being built"
fi
