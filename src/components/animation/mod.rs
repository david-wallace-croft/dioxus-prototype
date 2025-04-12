use self::animator::Animator;
use ::com_croftsoft_lib_animation::web_sys::spawn_local_loop;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::std::sync::Arc;
use ::std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
use ::tracing::info;

mod animator;
mod color;

const CANVAS_ID: &str = "home-page-canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  // TODO: Pause animation when browser window minimized

  static CSS: Asset = asset!("/assets/animation/app-animation.css");

  let mut click_count: i32 = 0;

  let request_blur: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  let request_drift: Arc<AtomicI8> = Arc::new(AtomicI8::new(0));

  let request_focus: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  let request_stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  let request_update: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  // TODO: Revisit when Dioxus supports async closures
  let request_blur_for_closure: Arc<AtomicBool> = request_blur.clone();

  let request_drift_for_closure: Arc<AtomicI8> = request_drift.clone();

  let request_focus_for_closure: Arc<AtomicBool> = request_focus.clone();

  let request_stop_for_closure: Arc<AtomicBool> = request_stop.clone();

  let request_update_for_closure: Arc<AtomicBool> = request_update.clone();

  let looper_closure = move || {
    spawn_animator(
      request_blur_for_closure.clone(),
      request_drift_for_closure.clone(),
      request_focus_for_closure.clone(),
      request_stop_for_closure.clone(),
      request_update_for_closure.clone(),
    )
  };

  use_future(looper_closure);

  use_drop(move || {
    request_stop.store(true, Ordering::SeqCst);
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
      height: "360",
      id: CANVAS_ID,
      onblur: move |_event| request_blur.store(true, Ordering::SeqCst),
      onclick: move |event| on_click(event, &mut click_count),
      onfocus: move |_event| request_focus.store(true, Ordering::SeqCst),
      onkeydown: move |_event| request_update.store(true, Ordering::SeqCst),
      onwheel: move |event| on_wheel(event, request_drift.clone()),
      tabindex: 0,
      width: "470",
    }
    }
  }
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

fn on_wheel(
  event: Event<WheelData>,
  request_drift: Arc<AtomicI8>,
) {
  let wheel_delta: WheelDelta = event.delta();

  let delta: f64 = match wheel_delta {
    Lines(lines_vector) => lines_vector.y,
    Pages(pages_vector) => pages_vector.y,
    Pixels(pixels_vector) => pixels_vector.y,
  };

  let drift_delta: i8 = delta.clamp(-128., 127.) as i8;

  request_drift.store(drift_delta, Ordering::SeqCst);
}

async fn spawn_animator(
  blur: Arc<AtomicBool>,
  drift: Arc<AtomicI8>,
  focus: Arc<AtomicBool>,
  stop: Arc<AtomicBool>,
  update: Arc<AtomicBool>,
) {
  let loop_updater = Animator::new(blur, CANVAS_ID, drift, focus, stop, update);

  spawn_local_loop(loop_updater);
}
