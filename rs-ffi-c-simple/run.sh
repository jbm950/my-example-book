#!/usr/bin/env bash

set -e  # Exit if any command fails

mkdir -p target/debug

gcc -c c/hello.c -o target/debug/hello.o

rustc src/main.rs -C link-arg=target/debug/hello.o -o target/debug/rs-ffi-c-simple

target/debug/rs-ffi-c-simple
