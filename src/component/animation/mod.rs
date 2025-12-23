use self::animator::Animator;
use self::constants::{
  CANVAS_BACKGROUND_COLOR, CANVAS_CURSOR, CANVAS_HEIGHT, CANVAS_ID,
  CANVAS_WIDTH, CSS,
};
use self::looper::Looper;
use self::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::spawn_local_loop;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::tracing::debug;

mod animator;
mod color;
mod constants;
mod frame_rater_updater_input;
mod looper;
mod user_input;

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  debug!("Animaton() component render");

  let mut user_input_signal: Signal<UserInput> = use_signal(Default::default);

  use_drop(move || {
    debug!("dropping");
  });

  use_future(move || async move {
    let animator: Animator = Animator::new(CANVAS_ID);

    let looper: Looper = Looper::new(animator, user_input_signal);

    spawn_local_loop(looper);
  });

  let onblur = move |_event: Event<FocusData>| {
    user_input_signal
      .with_mut(|user_input: &mut UserInput| user_input.blur = true);
  };

  let onclick = move |_event: Event<MouseData>| {
    user_input_signal
      .with_mut(|user_input: &mut UserInput| user_input.click = true);
  };

  let onfocus = move |_event: Event<FocusData>| {
    user_input_signal
      .with_mut(|user_input: &mut UserInput| user_input.focus = true);
  };

  let onkeydown = move |_event: Event<KeyboardData>| {
    user_input_signal
      .with_mut(|user_input: &mut UserInput| user_input.play = true);
  };

  let onkeyup = move |_event: Event<KeyboardData>| {
    user_input_signal
      .with_mut(|user_input: &mut UserInput| user_input.pause = true);
  };

  let onwheel = move |event: Event<WheelData>| {
    let wheel_delta: WheelDelta = event.delta();

    let delta: f64 = match wheel_delta {
      Lines(lines_vector) => lines_vector.y,
      Pages(pages_vector) => pages_vector.y,
      Pixels(pixels_vector) => pixels_vector.y,
    };

    let drift_delta: i8 = delta.clamp(i8::MIN as f64, i8::MAX as f64) as i8;

    user_input_signal
      .with_mut(|user_input: &mut UserInput| user_input.drift = drift_delta);
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
      background_color: CANVAS_BACKGROUND_COLOR,
      cursor: CANVAS_CURSOR,
      height: CANVAS_HEIGHT,
      id: CANVAS_ID,
      onblur,
      onclick,
      onfocus,
      onkeydown,
      onkeyup,
      onwheel,
      tabindex: 0,
      width: CANVAS_WIDTH,
    }
    }
  }
}
