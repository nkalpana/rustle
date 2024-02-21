# rustle

Build Wordle in Rustle

# Dev setup

## Install Rust

https://www.rust-lang.org/tools/install

## Install WASM pack

cargo install wasm-pack

## Install miniserve

cargo install miniserve

# Build project

## WASM Build

At rustle project level:

wasm-pack build --target web

## Start miniserve

At rustle project level:

miniserve .

Go to localhost:8080/index.html
