use super::controller::Controller;
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::dioxus::prelude::*;
use ::tracing::debug;

pub struct Looper {
  controller: Controller,
  user_input_signal: Signal<UserInput>,
}

impl Looper {
  pub fn new(
    controller: Controller,
    user_input_signal: Signal<UserInput>,
  ) -> Self {
    Self {
      controller,
      user_input_signal,
    }
  }
}

impl LoopUpdater for Looper {
  fn update_loop(
    &mut self,
    frame_time_millis: f64,
  ) -> bool {
    if self.user_input_signal.try_write().is_err() {
      // Stop looping when the component and its signals have been dropped

      debug!("stopping");

      return true;
    };

    // Take the user input and replace it with the default values to reset

    let user_input: UserInput = self.user_input_signal.take();

    self.controller.update(frame_time_millis, &user_input);

    false
  }
}
