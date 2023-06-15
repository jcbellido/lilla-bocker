/// This is shamelessly taken from nannou's: examples/draw/draw_capture.rs
///   the idea here is to render to texture a number of procedurally generated images
///   other samples, like the one under: examples/draw/draw_capture_hi_res.rs might have other clues
///   on how to work with larger images
use std::sync::OnceLock;

use anyhow::Result;

use nannou::prelude::*;

use crate::args::{ImageSize, PageRange};

#[derive(Clone, Debug)]
struct NannouExtraParameters {
    pub page_range: PageRange,
    pub image_size: ImageSize,
    pub path: String,
}

static NANNOU_EXTRA_PARAMETERS: OnceLock<NannouExtraParameters> = OnceLock::new();

pub fn build_images(path: &str, image_size: &ImageSize, page_range: &PageRange) -> Result<()> {
    tracing::info!("Starting rendering: output path {:#?}", path);
    tracing::info!("Starting rendering: {:#?}", image_size);

    let extra_params = NannouExtraParameters {
        path: path.to_string(),
        image_size: image_size.clone(),
        page_range: page_range.clone(),
    };

    NANNOU_EXTRA_PARAMETERS
        .set(extra_params)
        .expect("Error initializing nannou's extra parameters");

    nannou::app(model).update(update).run();

    // I need to execute this a number of times, I could use the update function to "randomize" the looks
    // When the max pages is finally reached, I could just "quit" and that should be it, no?

    Ok(())
}

#[derive(Clone, Debug)]
struct Model {
    pub extra: NannouExtraParameters,
    pub _window: window::Id,
}

fn model(app: &App) -> Model {
    let extra = NANNOU_EXTRA_PARAMETERS
        .get()
        .expect("Nannou's extra parameters missing?")
        .clone();

    let _window = app
        .new_window()
        .size(
            extra.image_size.width as u32,
            extra.image_size.height as u32,
        )
        .view(my_view)
        .build()
        .expect("error creating Nannou's window");

    Model { extra, _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn my_view(app: &App, model: &Model, frame: Frame) {
    if frame.nth() >= model.extra.page_range.max as u64 {
        app.quit();
    }

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
    let file_path = captured_frame_path(model, &frame);
    tracing::info!("Will persist on: {:#?}", file_path);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(model: &Model, frame: &Frame) -> std::path::PathBuf {
    std::path::Path::new(&model.extra.path)
        .join(format!("{:03}", frame.nth()))
        .with_extension("png")
}
