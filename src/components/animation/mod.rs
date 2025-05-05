use self::animator::Animator;
use self::user_input::UserInput;
use ::com_croftsoft_lib_animation::web_sys::spawn_local_loop;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::tracing::debug;

mod animator;
mod color;
mod frame_rater_updater_input;
mod user_input;

const CANVAS_ID: &str = "home-page-canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  debug!("Animaton() component render");

  static CSS: Asset = asset!("/assets/animation/app-animation.css");

  let mut user_input_signal: Signal<UserInput> =
    use_signal(|| Default::default());

  use_drop(move || {
    debug!("dropping");
  });

  use_future(move || async move {
    // TODO: split into Controller and Looper using Slideshow as an example

    let loop_updater: Animator = Animator::new(CANVAS_ID, user_input_signal);

    spawn_local_loop(loop_updater);
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

    let drift_delta: i8 = delta.clamp(-128., 127.) as i8;

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
