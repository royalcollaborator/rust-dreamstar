use wasm_bindgen::prelude::*;
use web_sys::{window, Document, File};

#[wasm_bindgen(module = "/assets/main.js")]
extern "C" {
    pub async fn recaptcha(site_key: &str, action_name: &str) -> JsValue;
    pub async fn get_blob(data: String) -> JsValue;
    pub async fn captureVideoFrame(video_id: &str) -> JsValue;
    pub async fn videoToImage(video_url: &str) -> JsValue;
    pub async fn uploadFile(url: String, file: File, file_extension: String) -> JsValue;
    pub async fn captureCanvasImg(id: &str) -> JsValue;
    pub fn createCanvasElement() -> JsValue;
    pub fn copy_clipboard(str : String) ->JsValue;
    pub async fn register_service_worker() -> JsValue;
    pub  fn installPWA() -> JsValue;
    pub fn isPwaInstalled() -> JsValue;
}
