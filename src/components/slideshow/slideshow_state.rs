use super::constants::{
  CONTROL_PANEL_DISPLAY_TIME, IMAGE_ASSETS, IMAGE_DISPLAY_TIME,
};
use ::dioxus::prelude::*;

pub struct SlideshowState {
  pub control_panel_time_remaining: u64,
  pub image_index: usize,
  pub image_source: Asset,
  pub image_time_remaining: u64,
}

impl SlideshowState {
  pub fn next_image(&mut self) {
    self.image_index = (self.image_index + 1) % IMAGE_ASSETS.len();

    self.image_source = IMAGE_ASSETS[self.image_index];

    self.image_time_remaining = IMAGE_DISPLAY_TIME;
  }
}

impl Default for SlideshowState {
  fn default() -> Self {
    Self {
      control_panel_time_remaining: CONTROL_PANEL_DISPLAY_TIME,
      image_index: 0,
      image_source: IMAGE_ASSETS[0],
      image_time_remaining: IMAGE_DISPLAY_TIME,
    }
  }
}
