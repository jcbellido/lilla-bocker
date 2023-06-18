use nannou::lyon;
use nannou::lyon::algorithms::path::math::Point;
use nannou::lyon::algorithms::path::PathSlice;
use nannou::lyon::algorithms::walk::{walk_along_path, RepeatedPattern};
use nannou::lyon::path::iterator::*;
use nannou::prelude::*;

use super::Model;

pub fn draw(app: &App, model: &Model, frame: &Frame) {
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
