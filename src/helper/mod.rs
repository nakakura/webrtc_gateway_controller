/*
Copyright (c) 2016 Sean McArthur
https://github.com/seanmonstar/reqwest/blob/master/LICENSE-MIT

This file is a part of reqwest crate.
https://github.com/seanmonstar/reqwest/blob/master/tests/support/mod.rs
*/

pub mod server;

// TODO: remove once done converting to new support server?
#[allow(unused)]
pub static DEFAULT_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
