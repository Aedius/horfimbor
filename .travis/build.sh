#!/bin/bash

FOO=${IS_WASM:=false}

if .travis/build-condition.sh $TRAVIS_COMMIT_RANGE $PROJECT; then
    cd $TRAVIS_BUILD_DIR/$PROJECT

    echo "$PROJECT is being built"

    if [[ "$IS_WASM" = true ]] ; then
        echo "build  --target wasm32-unknown-unknown --verbose --all"
        cargo build  --target wasm32-unknown-unknown --verbose --all
    else
        echo "build --verbose --all"
        cargo build --verbose --all
    fi

    echo "$PROJECT is being tested"

    if [[ "$IS_WASM" = true ]] ; then
        echo "cargo test --target wasm32-unknown-unknown --verbose --all"
        cargo test --target wasm32-unknown-unknown --verbose --all --lib
    else
        echo "cargo test --verbose --all"
        cargo test --verbose --all
    fi

    echo "$PROJECT is done"

else
    echo "$PROJECT is NOT being built"
fi
