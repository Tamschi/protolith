//! Mostly typesafe largely static builders, geared for use in more complex macros.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fprotolith)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fprotolith)

#![doc(html_root_url = "https://docs.rs/protolith/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned)]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

/// Exposes [`Build::EmptyBuilder`], which is [`Default`].
pub trait Build {
	type EmptyBuilder: Default;
}
