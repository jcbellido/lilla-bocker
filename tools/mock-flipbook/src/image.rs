/// This is shamelessly taken from nannou's:
///   1. examples/draw/draw_capture.rs
///   1. generative_design/type/p_3_2_1_01.rs
///   the idea here is to render to texture a number of procedurally generated images
/// For larger image sizes other samples, like the one under: examples/draw/draw_capture_hi_res.rs
///   might have clues on how to operate with larger surfaces
use std::sync::OnceLock;

use anyhow::Result;

use nannou::prelude::*;

use crate::args::{ImageSize, PageRange};

mod background;
mod model;
mod text;

use model::*;

static NANNOU_EXTRA_PARAMETERS: OnceLock<NannouExtraParameters> = OnceLock::new();

pub fn build_images(
    path_images: &str,
    path_miniatures: &str,
    image_size: &ImageSize,
    page_range: &PageRange,
    num_flipbooks: u32,
    path_output: String,
) -> Result<()> {
    tracing::info!("Starting rendering: output path {:#?}", path_images);
    tracing::info!("Starting rendering: {:#?}", image_size);

    let extra_params = NannouExtraParameters {
        path_images: path_images.to_string(),
        path_miniatures: path_miniatures.to_string(),
        image_size: image_size.clone(),
        page_range: page_range.clone(),
        num_flipbooks,
        path_output,
    };

    NANNOU_EXTRA_PARAMETERS
        .set(extra_params)
        .expect("Error initializing nannou's extra parameters");

    tracing::info!("Starting nannou ...");
    nannou::app(model).update(update).exit(exit).run();
    tracing::info!("... nannou stopped");
    Ok(())
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

    Model {
        extra,
        _window,
        update_number: 0,
    }
}

fn exit(_app: &App, model: Model) {
    tracing::info!("nannou's exit function called");

    let source = std::path::PathBuf::from(model.extra.path_images);
    let target = std::path::PathBuf::from(model.extra.path_miniatures);
    crate::miniature::generate_miniatures(&source, &target)
        .expect("miniatures generation failed!?");
    let path_output = std::path::PathBuf::from(model.extra.path_output);
    crate::generate_flipbooks(
        model.extra.num_flipbooks,
        path_output,
        model.extra.page_range,
    )
    .expect("Error generating flipbooks");
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update_number += 1;
    if model.update_number > model.extra.page_range.max as usize {
        tracing::info!("Awaiting for all jobs to finish");
        app.main_window()
            .await_capture_frame_jobs()
            .expect("Error waiting for capture frame jobs");
        app.quit();
        tracing::info!("Quit executed");
    }
}

fn my_view(app: &App, model: &Model, frame: Frame) {
    background::draw(app, &frame);
    text::draw(app, model, &frame);
    // Capture the frame!
    let file_path = captured_frame_path(model, &frame);
    tracing::info!("Will persist on: {:#?}", file_path);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(model: &Model, frame: &Frame) -> std::path::PathBuf {
    std::path::Path::new(&model.extra.path_images)
        .join(format!("{:03}", frame.nth()))
        .with_extension(crate::generator_constants::IMAGES_EXT)
}
