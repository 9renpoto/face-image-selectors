#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use opencv::{
    core::{self, Vector},
    imgcodecs, imgproc,
    objdetect::CascadeClassifier,
    prelude::*,
    types, videoio,
};
use std::{
    env,
    sync::{Arc, Mutex},
};
use tauri::State;

struct Camera(Arc<Mutex<videoio::VideoCapture>>);

#[tauri::command]
async fn on_trigger(cam: State<'_, Camera>) -> Result<Vec<u8>, ()> {
    let mut cam = cam.0.lock().unwrap();
    let mut image = Mat::default();
    let dir = env::current_dir().unwrap();
    let mut face_cascade = CascadeClassifier::new(
        dir.join("haarcascade_frontalface_default.xml")
            .to_str()
            .unwrap(),
    )
    .unwrap();
    let mut smile_cascade =
        CascadeClassifier::new(dir.join("haarcascade_smile.xml").to_str().unwrap()).unwrap();

    // 画像取得
    cam.read(&mut image).unwrap();
    let mut buf = Vector::default();

    let mut gray = Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    let mut faces = types::VectorOfRect::new();
    face_cascade
        .detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            5,
            0,
            core::Size::new(0, 0),
            core::Size::new(0, 0),
        )
        .unwrap();

    for face in faces.iter() {
        let center = core::Point {
            x: face.x + face.width / 2,
            y: face.y + face.height / 2,
        };
        imgproc::circle(
            &mut image,
            center,
            (face.width / 2) as i32,
            core::Scalar::new(255.0, 0.0, 0.0, 0.0),
            2,
            8,
            0,
        )
        .unwrap();

        let roi_gray = Mat::roi(&gray, face).unwrap();
        let mut smiles: Vector<core::Rect_<i32>> = types::VectorOfRect::new();
        smile_cascade
            .detect_multi_scale(
                &roi_gray,
                &mut smiles,
                1.2,
                10,
                0,
                core::Size::new(20, 20),
                core::Size::new(0, 0),
            )
            .unwrap();

        if !smiles.is_empty() {
            for smile in smiles.iter() {
                let smile_center = core::Point {
                    x: face.x + smile.x + smile.width / 2,
                    y: face.y + smile.y + smile.height / 2,
                };
                imgproc::circle(
                    &mut image,
                    smile_center,
                    (smile.width / 2) as i32,
                    core::Scalar::new(0.0, 0.0, 255.0, 0.0),
                    2,
                    8,
                    0,
                )
                .unwrap();
            }
        }
    }

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
