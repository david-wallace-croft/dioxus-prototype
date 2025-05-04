use super::controller::Controller;
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::dioxus::prelude::*;
use ::tracing::debug;

pub struct Looper {
  controller: Controller,
  time_new: f64,
  time_old: f64,
  user_input_signal: Signal<UserInput>,
}

impl Looper {
  pub fn new(
    controller: Controller,
    user_input_signal: Signal<UserInput>,
  ) -> Self {
    Self {
      controller,
      time_new: 0.,
      time_old: 0.,
      user_input_signal,
    }
  }
}

impl LoopUpdater for Looper {
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

    self.controller.update(delta_time, user_input);

    false
  }
}
