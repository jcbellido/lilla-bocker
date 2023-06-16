/// This is shamelessly taken from nannou's:
///   1. examples/draw/draw_capture.rs
///   1. generative_design/type/p_3_2_1_01.rs
///   the idea here is to render to texture a number of procedurally generated images
/// For larger image sizes other samples, like the one under: examples/draw/draw_capture_hi_res.rs
///   might have clues on how to operate with larger surfaces
use std::sync::OnceLock;

use anyhow::Result;

use nannou::lyon;
use nannou::lyon::algorithms::path::math::Point;
use nannou::lyon::algorithms::path::PathSlice;
use nannou::lyon::algorithms::walk::{walk_along_path, RepeatedPattern};
use nannou::lyon::path::iterator::*;
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

    Ok(())
}

#[derive(Clone, Debug)]
struct Model {
    pub extra: NannouExtraParameters,
    pub _window: window::Id,
    pub update_number: usize,
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

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update_number += 1;
    if model.update_number > model.extra.page_range.max as usize {
        app.quit();
    }
}

fn my_view(app: &App, model: &Model, frame: Frame) {
    draw_background(app, &frame);
    draw_text(app, model, &frame);
    // Capture the frame!
    let file_path = captured_frame_path(model, &frame);
    tracing::info!("Will persist on: {:#?}", file_path);
    app.main_window().capture_frame(file_path);
}

fn draw_text(app: &App, model: &Model, frame: &Frame) {
    let page_number_as_text = format!("{:03}", model.update_number);

    // Where win_rect is going to control where we draw
    let win_rect = app.main_window().rect().pad_left(20.0);
    draw_text_in_rect(app, &page_number_as_text, win_rect, frame);

    for sub_rect in app.main_window().rect().subdivisions_iter() {
        draw_text_in_rect(app, &page_number_as_text, sub_rect, frame);
        for sub_sub_rect in sub_rect.subdivisions_iter() {
            draw_text_in_rect(app, &page_number_as_text, sub_sub_rect, frame);
        }
    }
}

fn draw_text_in_rect(app: &App, page_number_as_text: &String, win_rect: Rect, frame: &Frame<'_>) {
    let draw = app.draw();

    let text = text(&page_number_as_text)
        .font_size(128)
        .center_justify()
        .build(win_rect);

    draw.path().fill().color(BLACK).events(text.path_events());

    let mut builder = lyon::path::Path::builder();
    for e in text.path_events() {
        builder.path_event(e);
    }
    let path: lyon::path::Path = builder.build();

    let mut path_points: Vec<lyon::path::math::Point> = Vec::new();
    dots_along_path(
        path.as_slice(),
        &mut path_points,
        12.5,
        app.elapsed_frames() as f32,
    );

    path_points.iter().enumerate().for_each(|(i, p)| {
        //Lines
        let l = 5.0;
        draw.line()
            .start(pt2(p.x + l, p.y - l))
            .end(pt2(p.x - l, p.y + l))
            .rgb(0.7, 0.61, 0.0);
        // Dots
        let q = map_range(i, 0, path_points.len(), 0.0, 1.0);
        if i % 2 == 0 {
            draw.ellipse()
                .x_y(p.x, p.y)
                .radius(map_range(
                    (i as f32 * 0.05 + app.time * 4.3).sin(),
                    -1.0,
                    1.0,
                    3.0,
                    9.0,
                ))
                .hsv(q, 1.0, 0.5);
        }
    });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_background(app: &App, frame: &Frame) {
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
}

fn captured_frame_path(model: &Model, frame: &Frame) -> std::path::PathBuf {
    std::path::Path::new(&model.extra.path)
        .join(format!("{:03}", frame.nth()))
        .with_extension("png")
}

fn dots_along_path(path: PathSlice, dots: &mut Vec<Point>, interval: f32, offset: f32) {
    use std::ops::Rem;
    let dot_spacing = map_range(interval, 0.0, 1.0, 0.025, 1.0);
    let mut pattern = RepeatedPattern {
        callback: &mut |position, _tangent, _distance| {
            dots.push(position);
            true // Return true to continue walking the path.
        },
        // Invoke the callback above at a regular interval of 3 units.
        intervals: &[dot_spacing], // 0.05],// 0.05],
        index: 0,
    };

    let tolerance = 0.01; // The path flattening tolerance.
    let start_offset = offset.rem(12.0 + dot_spacing); // Start walking at the beginning of the path.
    walk_along_path(path.iter().flattened(tolerance), start_offset, &mut pattern);
}
