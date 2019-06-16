#!/bin/bash

if .travis/build-condition.sh $TRAVIS_COMMIT_RANGE $PROJECT; then
    cd $TRAVIS_BUILD_DIR/$PROJECT
    echo "$PROJECT is being built"
    cargo build --verbose --all
    echo "$PROJECT is being tested"
    cargo test --verbose --all
    echo "$PROJECT is done"
else
    echo "$PROJECT is NOT being built"
fi
