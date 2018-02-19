#! /bin/bash
set -e

if [[ $# -lt 1 ]] ; then
    echo "Usage: $(basename $0) TARGET"
    exit 1
fi

target=$1
mkdir -p systems
pushd ../systems
# @Todo 
# - add a check for the toolchain
# - allow --release as an option 
case "${target}" in
    "linux")
        echo "Building for 64 bit linux"
        cargo build --target=x86_64-unknown-linux-musl
        cp target/debug/libsystems.so ../client/systems/
        cp target/debug/systems ../client/systems/
        ;;
    "darwin")
        # @Fixme problem compiling on linux
        echo "Building for 64 bit mac"
        cargo build --target=x86_64-apple-darwin
        cp target/debug/libsystems.dylib ../client/systems/
        cp target/debug/systems ../client/systems/
        ;;
    *)
        echo "unknown target ${target}"
        exit 1
        ;;
esac
