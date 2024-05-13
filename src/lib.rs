#![deny(clippy::all)]
#![deny(rustdoc::broken_intra_doc_links)]
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(inline)]
pub use api::*;

mod api;
pub mod protocol;
