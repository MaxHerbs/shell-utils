# Shell-Utils

This repository contains a set of shell utilities for convenience in scripting.

## The Structure

This is structured as a ***mono-binary*** (yes I made that up) - a single binary that contains multiple tools and behaves like multiple binaries. Running `shell-utils --expand` will build all the currently supported symlinks and add them to PATH, and then calling a given tool by name will run it.

The main entry point to the repo is `./src/shell_utils.rs`, but at runtime `main.rs` can choose other entrypoints, based on the name of

In a vanilla install, there is no `main.rs` - instead there is a `main.jinja`, and an empty `lib.rs`. The `lib.rs` is provided to keep the compiler happy as `cargo` will not build without a main. At compile time, the compiler will run `build.rs` and create/update main.rs.

> [!IMPORTANT]
> You must compile twice on the first build/everytime a change is made to `main.jinja`, or you will be compiling against the previous version.

## Running the Tool

### Building from Source
