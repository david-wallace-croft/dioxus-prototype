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

  // TODO: Is it better to use Arc<AtomicBool> instead of Signal<bool>?

  let mut blur_signal: Signal<bool> = use_signal(|| false);

  let mut focus_signal: Signal<bool> = use_signal(|| false);

  let mut update_signal: Signal<bool> = use_signal(|| false);

  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/use_effect.rs
  use_future(move || async move {
    let mut animator = Animator::new(CANVAS_ID, MESSAGE_START);

    let mut repaint = false;

    let mut running = true;

    let mut update = false;

    loop {
      if *blur_signal.read() {
        blur_signal.set(false);

        animator.set_message(MESSAGE_START);

        running = true;
      }

      if *focus_signal.read() {
        focus_signal.set(false);

        animator.set_message(MESSAGE_CONTROLS);

        repaint = true;

        running = false;
      }

      if *update_signal.read() {
        update_signal.set(false);

        update = true;
      }

      if running || update {
        update = false;

        animator.update();

        repaint = true;
      }

      if repaint {
        repaint = false;

        animator.paint();
      }

      async_std::task::sleep(Duration::from_millis(17u64)).await;
    }
  });

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    div {
      class: "app-fade-in-animation box",
    h1 {
      "Animation"
    }
    canvas {
      background_color: "black",
      cursor: "crosshair",
      id: CANVAS_ID,
      // https://docs.rs/dioxus/latest/dioxus/events/index.html
      onblur: move |_event| blur_signal.set(true),
      onclick: move |event| on_click(event, &mut click_count),
      onfocus: move |_event| focus_signal.set(true),
      onkeydown: move |_event| update_signal.set(true),
      onwheel: move |_event| update_signal.set(true),
      tabindex: 0,
      width: "600",
    }
    }
  }
}

fn on_click(
  _event: Event<MouseData>,
  click_count: &mut i32,
) {
  *click_count += 1;

  info!("click count: {click_count:?}");
}

// fn on_wheel(
//   _event: Event<WheelData>,
//   // color_signal: &mut Signal<Color>,
//   update_signal: &mut Signal<bool>,
// ) {
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

//   update_signal.set(true);
// }
