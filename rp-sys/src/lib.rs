#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("impl.rs");
