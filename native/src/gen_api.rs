#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.78.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_init_predictor_impl(port_: MessagePort, model_path: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init_predictor",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_model_path = model_path.wire2api();
            move |task_callback| init_predictor(api_model_path)
        },
    )
}
fn wire_predict_impl(port_: MessagePort, bytes: impl Wire2Api<Vec<f32>> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "predict",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_bytes = bytes.wire2api();
            move |task_callback| predict(api_bytes)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "gen_api.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
