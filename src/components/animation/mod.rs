use self::animator::Animator;
use self::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::spawn_local_loop;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::std::cell::RefCell;
use ::std::rc::Rc;
use ::tracing::debug;

mod animator;
mod color;
mod user_input;

const CANVAS_ID: &str = "home-page-canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  static CSS: Asset = asset!("/assets/animation/app-animation.css");

  let user_input: Rc<RefCell<UserInput>> =
    Rc::new(RefCell::new(UserInput::default()));

  let looper_closure_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  let looper_closure =
    move || spawn_animator(looper_closure_user_input.clone());

  use_future(looper_closure);

  let drop_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  use_drop(move || {
    debug!("dropping");

    drop_user_input.borrow_mut().stop = true;
  });

  let blur_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  let click_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  let focus_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  let keydown_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  let keyup_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  let wheel_user_input: Rc<RefCell<UserInput>> = user_input.clone();

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    div {
      class: "app-animation app-fade-in-animation box",
    h1 {
    "Animation"
    }
    canvas {
      background_color: "black",
      cursor: "crosshair",
      height: "360",
      id: CANVAS_ID,
      onblur: move |event| on_blur(event, blur_user_input.clone()),
      onclick: move |event| on_click(event, click_user_input.clone()),
      onfocus: move |event| on_focus(event, focus_user_input.clone()),
      onkeydown: move |event| on_keydown(event, keydown_user_input.clone()),
      onkeyup: move |event| on_keyup(event, keyup_user_input.clone()),
      onwheel: move |event| on_wheel(event, wheel_user_input.clone()),
      tabindex: 0,
      width: "470",
    }
    }
  }
}

fn on_blur(
  _event: Event<FocusData>,
  user_input: Rc<RefCell<UserInput>>,
) {
  user_input.borrow_mut().blur = true;
}

fn on_click(
  _event: Event<MouseData>,
  user_input: Rc<RefCell<UserInput>>,
) {
  user_input.borrow_mut().click = true;
}

fn on_focus(
  _event: Event<FocusData>,
  user_input: Rc<RefCell<UserInput>>,
) {
  user_input.borrow_mut().focus = true;
}

fn on_keydown(
  _event: Event<KeyboardData>,
  user_input: Rc<RefCell<UserInput>>,
) {
  user_input.borrow_mut().play = true;
}

fn on_keyup(
  _event: Event<KeyboardData>,
  user_input: Rc<RefCell<UserInput>>,
) {
  user_input.borrow_mut().pause = true;
}

fn on_wheel(
  event: Event<WheelData>,
  user_input: Rc<RefCell<UserInput>>,
) {
  let wheel_delta: WheelDelta = event.delta();

  let delta: f64 = match wheel_delta {
    Lines(lines_vector) => lines_vector.y,
    Pages(pages_vector) => pages_vector.y,
    Pixels(pixels_vector) => pixels_vector.y,
  };

  let drift_delta: i8 = delta.clamp(-128., 127.) as i8;

  user_input.borrow_mut().drift = drift_delta;
}

async fn spawn_animator(user_input: Rc<RefCell<UserInput>>) {
  let loop_updater = Animator::new(CANVAS_ID, user_input);

  spawn_local_loop(loop_updater);
}
