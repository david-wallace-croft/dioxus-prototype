use super::constants::{
  CONTROL_PANEL_DISPLAY_TIME, CONTROL_PANEL_FADE_TIME, IMAGE_ASSETS,
  IMAGE_DISPLAY_TIME,
};
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::dioxus::prelude::*;
use ::tracing::debug;

pub struct Controller {
  control_panel_fade_signal: Signal<bool>,
  control_panel_show_signal: Signal<bool>,
  control_panel_time_remaining: f64,
  image_index: usize,
  image_source_signal: Signal<Asset>,
  image_time_remaining: f64,
  time_new: f64,
  time_old: f64,
  user_input_signal: Signal<UserInput>,
}

impl Controller {
  pub fn new(
    control_panel_fade_signal: Signal<bool>,
    control_panel_show_signal: Signal<bool>,
    image_source_signal: Signal<Asset>,
    user_input_signal: Signal<UserInput>,
  ) -> Self {
    Self {
      control_panel_fade_signal,
      control_panel_show_signal,
      control_panel_time_remaining: CONTROL_PANEL_DISPLAY_TIME,
      image_index: 0,
      image_source_signal,
      image_time_remaining: IMAGE_DISPLAY_TIME,
      time_new: 0.,
      time_old: 0.,
      user_input_signal,
    }
  }

  fn next_image(&mut self) {
    self.image_time_remaining = IMAGE_DISPLAY_TIME;

    self.image_index = (self.image_index + 1) % IMAGE_ASSETS.len();

    self.image_source_signal.set(IMAGE_ASSETS[self.image_index]);
  }
}

impl LoopUpdater for Controller {
  fn update_loop(
    &mut self,
    update_time: f64,
  ) -> bool {
    if self.user_input_signal.try_write().is_err() {
      // Stop looping when the component and its signals have been dropped

      debug!("stopping");

      return true;
    };

    // Take the user input and replace it with the default values to reset

    let user_input: UserInput = self.user_input_signal.take();

    self.time_old = self.time_new;

    self.time_new = update_time;

    let mut delta_time: f64 = self.time_new - self.time_old;

    if delta_time >= 1_000. {
      delta_time = 0.;
    }

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

    false
  }
}
