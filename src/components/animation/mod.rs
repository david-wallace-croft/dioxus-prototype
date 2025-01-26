use self::animator::Animator;
// use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::std::time::Duration;
use ::tracing::info;

mod animator;
mod color;

const CANVAS_ID: &str = "home-page-canvas";

const MESSAGE_CONTROLS: &str = "Hold a key or scroll the mouse wheel";

const MESSAGE_START: &str = "Click on or tab to the canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  static CSS: Asset = asset!("/assets/animation/app-animation.css");

  let mut click_count: i32 = 0;

  let mut message_signal: Signal<&str> = use_signal(|| MESSAGE_START);

  let mut running_signal: Signal<bool> = use_signal(|| true);

  let mut update_signal: Signal<bool> = use_signal(|| true);

  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/use_effect.rs
  use_future(move || {
    to_owned![message_signal];
    to_owned![running_signal];
    to_owned![update_signal];
    async move {
      let mut animator = Animator::new(CANVAS_ID, MESSAGE_START.into());

      loop {
        if *running_signal.read() || *update_signal.read() {
          update_signal.set(false);

          // TODO: move this to event handlers
          animator.set_message(*message_signal.read());

          animator.update();

          animator.paint();
        }

        async_std::task::sleep(Duration::from_millis(17u64)).await;
      }
    }
  });

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    div {
      class: "app-animation box",
    h1 {
      "Animation"
    }
    canvas {
      background_color: "black",
      cursor: "crosshair",
      id: CANVAS_ID,
      // https://docs.rs/dioxus/latest/dioxus/events/index.html
      onblur: move |event| on_blur(event, &mut message_signal, &mut running_signal),
      onclick: move |event| on_click(event, &mut click_count, &mut update_signal),
      onfocus: move |event| on_focus(event, &mut message_signal, &mut running_signal, &mut update_signal),
      onkeydown: move |event| on_key_down(event, &mut update_signal),
      onwheel: move |event| on_wheel(event, &mut update_signal),
      tabindex: 0,
      width: "600",
    }
    }
  }
}

fn on_blur(
  _event: Event<FocusData>,
  message_signal: &mut Signal<&str>,
  running_signal: &mut Signal<bool>,
) {
  // log::info!("onblur Event: {event:?}");

  message_signal.set(MESSAGE_START);

  running_signal.set(true);
}

fn on_click(
  _event: Event<MouseData>,
  click_count: &mut i32,
  _update_signal: &mut Signal<bool>,
) {
  *click_count = *click_count + 1;

  info!("click count: {click_count:?}");
}

fn on_focus(
  _event: Event<FocusData>,
  message_signal: &mut Signal<&str>,
  running_signal: &mut Signal<bool>,
  update_signal: &mut Signal<bool>,
) {
  // log::info!("onfocus Event: {event:?}");

  message_signal.set(MESSAGE_CONTROLS);

  running_signal.set(false);

  // TODO: change this to repaint_signal

  update_signal.set(true);
}

fn on_key_down(
  _event: Event<KeyboardData>,
  update_signal: &mut Signal<bool>,
) {
  update_signal.set(true);
}

fn on_wheel(
  _event: Event<WheelData>,
  // color_signal: &mut Signal<Color>,
  update_signal: &mut Signal<bool>,
) {
  // log::info!("onwheel Event: {event:?}");

  // let wheel_delta: WheelDelta = event.delta();

  // let delta: f64 = match wheel_delta {
  //   Lines(lines_vector) => lines_vector.y,
  //   Pages(pages_vector) => pages_vector.y,
  //   Pixels(pixels_vector) => pixels_vector.y,
  // };

  // let delta: i8 = delta.clamp(-1., 1.) as i8;

  // let color: Color = *color_signal.read();

  // color_signal.set(color.shift(delta));

  update_signal.set(true);
}
