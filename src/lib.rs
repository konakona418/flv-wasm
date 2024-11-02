mod utils;
mod flv_rs;

use std::collections::VecDeque;
use flv_rs_single::exchange::RemuxedData;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static mut FLV_RS: Option<flv_rs::FlvRs> = None;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, flv-wasm!");
}

#[wasm_bindgen]
pub fn init_local() -> Result<(), JsError> {
    unsafe {
        FLV_RS = Some(flv_rs::FlvRs::new());
    }
    Ok(())
}

#[wasm_bindgen]
pub fn push_data(data: JsValue) -> Result<(), JsError> {
    let data = js_sys::Uint8Array::new(&data);
    let data: Vec<u8> = data.to_vec();
    match unsafe {
        FLV_RS.as_mut().unwrap().push_data(&mut VecDeque::from(data))
    } {
        Ok(_) => Ok(()),
        Err(e) => Err(JsError::new(&e.to_string()))
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsError> {
    match unsafe {
        FLV_RS.as_mut().unwrap().start()
    } {
        Ok(_) => Ok(()),
        Err(e) => Err(JsError::new(&e.to_string()))
    }
}

#[wasm_bindgen]
pub fn continue_decoding() -> Result<(), JsError> {
    match unsafe {
        FLV_RS.as_mut().unwrap().continue_decoding()
    } {
        Ok(_) => Ok(()),
        Err(e) => Err(JsError::new(&e.to_string()))
    }
}

#[wasm_bindgen]
pub fn stop() -> Result<(), JsError> {
    match unsafe {
        FLV_RS.as_mut().unwrap().stop()
    } {
        Ok(_) => Ok(()),
        Err(e) => Err(JsError::new(&e.to_string()))
    }
}

/// Return the codec configuration
/// The result is an array of 2 elements:
/// - The first element is the audio codec configuration
/// - The second element is the video codec configuration
#[wasm_bindgen]
pub fn get_codec() -> Result<JsValue, JsError> {
    match unsafe {
        FLV_RS.as_mut().unwrap().get_codec_conf()
    } {
        Ok(conf) => {
            let (audio, video) = conf;
            Ok(
                js_sys::Array::of2(
                    &JsValue::from(audio),
                    &JsValue::from(video)
                ).into()
            )
        },
        Err(e) => Err(JsError::new(&e.to_string()))
    }
}

/// Return the next data to be sent to the player
/// An array of 2 elements is returned:
/// - The first element is the media type (0: header, 1: video, 2: audio)
/// - The second element is the payload
#[wasm_bindgen]
pub fn consume() -> Result<JsValue, JsError> {
    match unsafe {
        FLV_RS.as_mut().unwrap().consume()
    } {
        Ok(data) => {
            let payload: Vec<u8>;
            let media_type = match data {
                RemuxedData::Header(data) => {
                    payload = data;
                    0
                },
                RemuxedData::Video(data) => {
                    payload = data;
                    1
                },
                RemuxedData::Audio(data) => {
                    payload = data;
                    2
                },
            };
            Ok(
                js_sys::Array::of2(
                    &JsValue::from(media_type),
                    &js_sys::Uint8Array::from(payload.as_slice()).into()
                ).into()
            )
        },
        Err(e) => Err(JsError::new(&e.to_string()))
    }
}
