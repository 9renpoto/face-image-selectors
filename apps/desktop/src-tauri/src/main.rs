#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use opencv::{core::Vector, imgcodecs, prelude::*, videoio};
use std::sync::{Arc, Mutex};
use tauri::State;

struct Camera(Arc<Mutex<videoio::VideoCapture>>);

#[tauri::command]
async fn on_trigger(cam: State<'_, Camera>) -> Result<Vec<u8>, ()> {
    let mut cam = cam.0.lock().unwrap();
    let mut image = Mat::default();
    // 画像取得
    cam.read(&mut image).unwrap();
    let mut buf = Vector::default();
    // エンコード
    imgcodecs::imencode(".png", &mut image, &mut buf, &Vector::default()).unwrap();
    Ok(buf.to_vec())
}

fn main() {
    let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    tauri::Builder::default()
        .manage(Camera(Arc::new(Mutex::new(cam))))
        .invoke_handler(tauri::generate_handler![on_trigger])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
