use nannou::prelude::*;

use crate::args::{ImageSize, PageRange};

#[derive(Clone, Debug)]
pub struct NannouExtraParameters {
    pub page_range: PageRange,
    pub image_size: ImageSize,
    pub path_images: String,
    pub path_miniatures: String,
    pub num_flipbooks: u32,
    pub path_output: String,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub extra: NannouExtraParameters,
    pub _window: window::Id,
    pub update_number: usize,
}
