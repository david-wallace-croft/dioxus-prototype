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
mod frame_rater_updater_input;
mod user_input;

const CANVAS_ID: &str = "home-page-canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  static CSS: Asset = asset!("/assets/animation/app-animation.css");

  // TODO: Switch to using Signal; use Slideshow as an example

  let user_input_0: Rc<RefCell<UserInput>> = Default::default();

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  use_drop(move || {
    debug!("dropping");

    user_input.borrow_mut().stop = true;
  });

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  use_future(move || {
    let user_input: Rc<RefCell<UserInput>> = user_input.clone();

    async move {
      let loop_updater = Animator::new(CANVAS_ID, user_input);

      spawn_local_loop(loop_updater);
    }
  });

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  let onblur =
    move |_event: Event<FocusData>| user_input.borrow_mut().blur = true;

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  let onclick =
    move |_event: Event<MouseData>| user_input.borrow_mut().click = true;

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  let onfocus =
    move |_event: Event<FocusData>| user_input.borrow_mut().focus = true;

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  let onkeydown =
    move |_event: Event<KeyboardData>| user_input.borrow_mut().play = true;

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  let onkeyup =
    move |_event: Event<KeyboardData>| user_input.borrow_mut().pause = true;

  let user_input: Rc<RefCell<UserInput>> = user_input_0.clone();

  let onwheel = move |event: Event<WheelData>| {
    let wheel_delta: WheelDelta = event.delta();

    let delta: f64 = match wheel_delta {
      Lines(lines_vector) => lines_vector.y,
      Pages(pages_vector) => pages_vector.y,
      Pixels(pixels_vector) => pixels_vector.y,
    };

    let drift_delta: i8 = delta.clamp(-128., 127.) as i8;

    user_input.borrow_mut().drift = drift_delta;
  };

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
      onblur,
      onclick,
      onfocus,
      onkeydown,
      onkeyup,
      onwheel,
      tabindex: 0,
      width: "470",
    }
    }
  }
}
