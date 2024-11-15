#!/bin/sh

local() {
#
if ! [ -d ./sut_native ]; then
    cargo new sut_native
fi

if ! [ -f ./caller/mix.exs ]; then
    mix new caller
    mkdir -pv ./caller/native/
else
    mkdir -pv ./caller/native/
    mv ./sut_native ./caller/native/
fi

cd ./caller || return
mix deps.get
mix compile

}
