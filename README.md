# mozilla-rust-sdk
Rust SDK for Google Cloud

## High-level overview
#### docs
This directory contains admin documention
on the project.  We can also use it as a
place to store customer facing docs about
design/usage/etc.

#### googleapis

This git submodule contains gRPC service
definitions for the various APIs that we
will be targeting in this project. They
can be fed into various code generation
utilities for generating native client
implementations in any language that has
created a generator.

#### pbgen

This directory contains a library that
exports mechanically generated gRPC
types for use with the BigTable, PubSub,
and Spanner endpoints. The library
generates native Rust interfaces for
these endpoints on every build by
invoking build logic from the `build.rs`
file in the crate's root, and then
exporting the generated code through
`src/lib.rs`.
