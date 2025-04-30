use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::std::cell::RefCell;
use ::std::rc::Rc;
use std::mem::take;
use tracing::debug;

pub struct InputHandler {
  time_new: f64,
  time_old: f64,
  user_input: Rc<RefCell<UserInput>>,
}

impl InputHandler {
  pub fn new(user_input: Rc<RefCell<UserInput>>) -> Self {
    Self {
      time_new: 0.,
      time_old: 0.,
      user_input,
    }
  }
}

impl LoopUpdater for InputHandler {
  fn update_loop(
    &mut self,
    update_time: f64,
  ) -> bool {
    debug!("update_time: {update_time}");

    self.time_old = self.time_new;

    self.time_new = update_time;

    // Take the user input and replace it with the default values to reset

    let user_input: UserInput = take(&mut *self.user_input.borrow_mut());

    // TODO: Move input handling into here

    let stopping: bool = user_input.stop;

    if stopping {
      debug!("stopping");
    }

    stopping
  }
}
