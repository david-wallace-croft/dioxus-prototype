use super::SlideshowState;
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::dioxus::prelude::*;
use ::std::cell::RefCell;
use ::std::mem::take;
use ::std::rc::Rc;
use ::tracing::debug;

pub struct Controller {
  slideshow_state_signal: Signal<SlideshowState>,
  time_new: f64,
  time_old: f64,
  user_input: Rc<RefCell<UserInput>>,
}

impl Controller {
  pub fn new(
    slideshow_state_signal: Signal<SlideshowState>,
    user_input: Rc<RefCell<UserInput>>,
  ) -> Self {
    Self {
      slideshow_state_signal,
      time_new: 0.,
      time_old: 0.,
      user_input,
    }
  }
}

impl LoopUpdater for Controller {
  fn update_loop(
    &mut self,
    update_time: f64,
  ) -> bool {
    // debug!("update_time: {update_time}");

    self.time_old = self.time_new;

    self.time_new = update_time;

    if let Ok(state) = self.slideshow_state_signal.try_read() {
      debug!("image time remaining: {}", state.image_time_remaining);
    }

    // Take the user input and replace it with the default values to reset

    let user_input: UserInput = take(&mut *self.user_input.borrow_mut());

    // TODO: Move input handling into here

    if user_input.skip {
      debug!("Controller.update_loop() skip");
    }

    let stopping: bool = user_input.stop;

    if stopping {
      debug!("stopping");
    }

    stopping
  }
}
