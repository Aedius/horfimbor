#!/bin/bash


if .travis/build-condition.sh $TRAVIS_COMMIT_RANGE $PROJECT; then
    echo "$PROJECT is being tested"
    cd $TRAVIS_BUILD_DIR/$PROJECT
    cargo test --verbose --all
else
    echo "$PROJECT is NOT being tested"
fi