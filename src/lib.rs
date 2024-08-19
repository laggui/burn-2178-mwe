#![cfg_attr(not(test), no_std)]

use burn::{
    backend::{
        wgpu::{init_async, AutoGraphicsApi, WgpuDevice},
        Wgpu,
    },
    tensor::Tensor,
};
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    // Init logger
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
pub async fn matmul() {
    type B = Wgpu;
    let device = WgpuDevice::default();
    init_async::<AutoGraphicsApi>(&device, Default::default()).await;

    let a = Tensor::<B, 2>::from_floats([[1., 2.], [3., 4.]], &device);
    let b = Tensor::<B, 2>::from_floats([[2., 3.], [4., 5.]], &device);

    // Expected: [[10., 13.], [22., 29.]]
    let c = a.matmul(b);

    // console.log
    let data = c.to_data_async().await;
    log::info!("Matmul result {:?}:\n{data}", data.shape);
}
