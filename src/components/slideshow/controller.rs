use super::constants::{
  CONTROL_PANEL_DISPLAY_TIME, CONTROL_PANEL_FADE_TIME, IMAGE_ASSETS,
  IMAGE_DISPLAY_TIME, MILLISECONDS_PER_SECOND,
};
use super::user_input::UserInput;
use ::dioxus::prelude::*;

pub struct Controller {
  control_panel_fade_signal: Signal<bool>,
  control_panel_show_signal: Signal<bool>,
  control_panel_time_remaining: f64,
  /// The timestamp of the current animation frame in milliseconds
  frame_time_millis_new: f64,
  /// The timestamp of the previous animation frame in milliseconds
  frame_time_millis_old: f64,
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
      frame_time_millis_new: 0.,
      frame_time_millis_old: 0.,
      image_index: 0,
      image_source_signal,
      image_time_remaining: IMAGE_DISPLAY_TIME,
    }
  }

  pub fn update(
    &mut self,
    frame_time_millis: f64,
    user_input: &UserInput,
  ) {
    self.frame_time_millis_old = self.frame_time_millis_new;

    self.frame_time_millis_new = frame_time_millis;

    let mut delta_time_millis: f64 =
      self.frame_time_millis_new - self.frame_time_millis_old;

    if delta_time_millis >= MILLISECONDS_PER_SECOND {
      delta_time_millis = 0.;
    }

    self.update_control_panel(delta_time_millis, user_input);

    self.update_image(delta_time_millis, user_input);
  }

  fn update_control_panel(
    &mut self,
    delta_time: f64,
    user_input: &UserInput,
  ) {
    if self.control_panel_time_remaining > 0. {
      self.control_panel_time_remaining -= delta_time;
    }

    if user_input.show {
      self.control_panel_time_remaining = CONTROL_PANEL_DISPLAY_TIME;
    }

    let fade: bool = *self.control_panel_fade_signal.read();

    if self.control_panel_time_remaining <= CONTROL_PANEL_FADE_TIME {
      if !fade {
        self.control_panel_fade_signal.set(true);
      }
    } else if fade {
      self.control_panel_fade_signal.set(false);
    }

    let show: bool = *self.control_panel_show_signal.read();

    if self.control_panel_time_remaining > 0. {
      if !show {
        self.control_panel_show_signal.set(true);
      }
    } else if show {
      self.control_panel_show_signal.set(false);
    }
  }

  fn update_image(
    &mut self,
    delta_time: f64,
    user_input: &UserInput,
  ) {
    self.image_time_remaining -= delta_time;

    if !user_input.skip && self.image_time_remaining > 0. {
      return;
    }

    self.image_time_remaining = IMAGE_DISPLAY_TIME;

    self.image_index = (self.image_index + 1) % IMAGE_ASSETS.len();

    self.image_source_signal.set(IMAGE_ASSETS[self.image_index]);
  }
}
