/* src/lib.rs
 * ==========
 *
 * Copying
 * -------
 *
 * Copyright (c) 2022 gitrs authors and contributors.
 *
 * This file is part of the *gitrs* project.
 *
 * gitrs is a free software project. You can redistribute it and/or modify it
 * following the terms of the MIT License.
 *
 * This software project is distributed *as is*, WITHOUT WARRANTY OF ANY KIND;
 * including but not limited to the WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE and NONINFRINGEMENT.
 *
 * You should have received a copy of the MIT License along with *gitrs*. If
 * not, see <http://opensource.org/licenses/MIT>.
 */
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! # git
//!
//! `git` is a pure Rust implementation of the Git core methods. This project is
//! currently unstable, use [git2](https://crates.io/crates/git2) in the
//! meanwhile.

/// Replicate the version of the package as provided by Cargo.
pub const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");
