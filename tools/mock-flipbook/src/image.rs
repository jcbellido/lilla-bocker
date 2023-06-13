/// This is shamelessly taken from nannou's: examples/draw/draw_capture.rs
///   the idea here is to render to texture a number of procedurally generated images
///   other samples, like the one under: examples/draw/draw_capture_hi_res.rs might have other clues
///   on how to work with larger images
use anyhow::Result;

use nannou::prelude::*;

use crate::args::{ImageSize, PageRange};

pub fn build_images(_path: &str, _image_size: &ImageSize, _page_range: &PageRange) -> Result<()> {
    nannou::sketch(view).run();
    Ok(())
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);

    let win = app.window_rect();
    draw.tri()
        .points(win.bottom_left(), win.top_left(), win.top_right())
        .color(VIOLET);

    let t = frame.nth() as f32 / 60.0;
    draw.ellipse()
        .x_y(app.mouse.x * t.cos(), app.mouse.y)
        .radius(win.w() * 0.125 * t.sin())
        .color(RED);

    draw.line()
        .weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
        .caps_round()
        .color(PALEGOLDENROD)
        .points(win.top_left() * t.sin(), win.bottom_right() * t.cos());

    draw.quad()
        .x_y(-app.mouse.x, app.mouse.y)
        .color(DARKGREEN)
        .rotate(t);

    draw.rect()
        .x_y(app.mouse.y, app.mouse.x)
        .w(app.mouse.x * 0.25)
        .hsv(t, 1.0, 1.0);

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    let file_path = captured_frame_path(app, &frame);
    tracing::info!("Will persist on: {:#?}", file_path);
    // app.main_window().capture_frame(file_path);
    app.quit();
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
