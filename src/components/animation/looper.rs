use super::animator::Animator;
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::dioxus::prelude::*;
use ::tracing::debug;

pub struct Looper {
  animator: Animator,
  user_input_signal: Signal<UserInput>,
}

impl Looper {
  pub fn new(
    animator: Animator,
    user_input_signal: Signal<UserInput>,
  ) -> Self {
    Self {
      animator,
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

    self.animator.update(update_time, &user_input);

    false
  }
}
