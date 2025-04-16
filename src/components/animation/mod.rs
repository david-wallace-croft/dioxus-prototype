use self::animator::Animator;
use self::inputs::Inputs;
use ::com_croftsoft_lib_animation::web_sys::spawn_local_loop;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::std::cell::RefCell;
use ::std::rc::Rc;
use ::tracing::{debug, info};

mod animator;
mod color;
mod inputs;

const CANVAS_ID: &str = "home-page-canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  // TODO: Verify that the animation pauses when the browser is minimized

  static CSS: Asset = asset!("/assets/animation/app-animation.css");

  // TODO: Move click_count into animator
  let mut click_count: i32 = 0;

  let inputs: Rc<RefCell<Inputs>> = Rc::new(RefCell::new(Inputs::default()));

  let looper_closure_inputs: Rc<RefCell<Inputs>> = inputs.clone();

  let looper_closure = move || spawn_animator(looper_closure_inputs.clone());

  use_future(looper_closure);

  let drop_inputs: Rc<RefCell<Inputs>> = inputs.clone();

  use_drop(move || {
    debug!("dropping");

    drop_inputs.borrow_mut().stop = true;
  });

  let blur_inputs: Rc<RefCell<Inputs>> = inputs.clone();

  let focus_inputs: Rc<RefCell<Inputs>> = inputs.clone();

  let keydown_inputs: Rc<RefCell<Inputs>> = inputs.clone();

  let keyup_inputs: Rc<RefCell<Inputs>> = inputs.clone();

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
      onblur: move |event| on_blur(event, blur_inputs.clone()),
      onclick: move |event| on_click(event, &mut click_count),
      onfocus: move |event| on_focus(event, focus_inputs.clone()),
      onkeydown: move |event| on_keydown(event, keydown_inputs.clone()),
      onkeyup: move |event| on_keyup(event, keyup_inputs.clone()),
      onwheel: move |event| on_wheel(event, inputs.clone()),
      tabindex: 0,
      width: "470",
    }
    }
  }
}

fn on_blur(
  _event: Event<FocusData>,
  inputs: Rc<RefCell<Inputs>>,
) {
  inputs.borrow_mut().blur = true;
}

fn on_click(
  // TODO: &mut self for click_count instead of passing it in?
  _event: Event<MouseData>,
  click_count: &mut i32,
) {
  *click_count += 1;

  // TODO: Animate the click count

  info!("click count: {click_count:?}");
}

fn on_focus(
  _event: Event<FocusData>,
  inputs: Rc<RefCell<Inputs>>,
) {
  inputs.borrow_mut().focus = true;
}

fn on_keydown(
  _event: Event<KeyboardData>,
  inputs: Rc<RefCell<Inputs>>,
) {
  inputs.borrow_mut().play = true;
}

fn on_keyup(
  _event: Event<KeyboardData>,
  inputs: Rc<RefCell<Inputs>>,
) {
  inputs.borrow_mut().pause = true;
}

fn on_wheel(
  event: Event<WheelData>,
  inputs: Rc<RefCell<Inputs>>,
) {
  let wheel_delta: WheelDelta = event.delta();

  let delta: f64 = match wheel_delta {
    Lines(lines_vector) => lines_vector.y,
    Pages(pages_vector) => pages_vector.y,
    Pixels(pixels_vector) => pixels_vector.y,
  };

  let drift_delta: i8 = delta.clamp(-128., 127.) as i8;

  inputs.borrow_mut().drift = drift_delta;
}

async fn spawn_animator(inputs: Rc<RefCell<Inputs>>) {
  let loop_updater = Animator::new(CANVAS_ID, inputs);

  spawn_local_loop(loop_updater);
}
