#!/usr/bin/env bash

if [[ -z $1 ]]; then
    echo "Build specified VST plugin"
    echo "Example:"
    echo -e "\t$0 distortion"
    echo -e "\tBuilds the distortion plugin"
else
    cargo build --manifest-path $1/Cargo.toml
fi
