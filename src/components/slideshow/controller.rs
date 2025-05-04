use super::constants::{
  CONTROL_PANEL_DISPLAY_TIME, CONTROL_PANEL_FADE_TIME, IMAGE_ASSETS,
  IMAGE_DISPLAY_TIME,
};
use super::user_input::UserInput;
use ::dioxus::prelude::*;

pub struct Controller {
  control_panel_fade_signal: Signal<bool>,
  control_panel_show_signal: Signal<bool>,
  control_panel_time_remaining: f64,
  image_index: usize,
  image_source_signal: Signal<Asset>,
  image_time_remaining: f64,
}

impl Controller {
  pub fn new(
    control_panel_fade_signal: Signal<bool>,
    control_panel_show_signal: Signal<bool>,
    image_source_signal: Signal<Asset>,
  ) -> Self {
    Self {
      control_panel_fade_signal,
      control_panel_show_signal,
      control_panel_time_remaining: CONTROL_PANEL_DISPLAY_TIME,
      image_index: 0,
      image_source_signal,
      image_time_remaining: IMAGE_DISPLAY_TIME,
    }
  }

  fn next_image(&mut self) {
    self.image_time_remaining = IMAGE_DISPLAY_TIME;

    self.image_index = (self.image_index + 1) % IMAGE_ASSETS.len();

    self.image_source_signal.set(IMAGE_ASSETS[self.image_index]);
  }

  pub fn update(
    &mut self,
    delta_time: f64,
    user_input: UserInput,
  ) {
    if self.control_panel_time_remaining > 0. {
      self.control_panel_time_remaining =
        self.control_panel_time_remaining - delta_time;
    }

    if user_input.show {
      self.control_panel_time_remaining = CONTROL_PANEL_DISPLAY_TIME;

      if *self.control_panel_fade_signal.read() {
        self.control_panel_fade_signal.set(false);
      }
    }

    if self.control_panel_time_remaining > 0. {
      if !*self.control_panel_show_signal.read() {
        self.control_panel_show_signal.set(true);
      }

      if self.control_panel_time_remaining < CONTROL_PANEL_FADE_TIME {
        if !*self.control_panel_fade_signal.read() {
          self.control_panel_fade_signal.set(true);
        }
      }
    } else {
      if *self.control_panel_fade_signal.read() {
        self.control_panel_fade_signal.set(false);
      }
      if *self.control_panel_show_signal.read() {
        self.control_panel_show_signal.set(false);
      }
    }

    self.image_time_remaining = self.image_time_remaining - delta_time;

    let mut select_next_image = false;

    if self.image_time_remaining <= 0. {
      select_next_image = true;
    }

    if user_input.skip {
      select_next_image = true;
    }

    if select_next_image {
      self.next_image();
    }
  }
}
