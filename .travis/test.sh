#!/bin/bash


if .travis/build-condition.sh $TRAVIS_COMMIT_RANGE $PROJECT; then
    echo "$PROJECT is being tested"
    cd $PROJET
    cargo test --verbose --all
else
    echo "$PROJECT is NOT being tested"
fi