use self::animator::Animator;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::std::time::Duration;
use ::tracing::info;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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

  let mut drift_signal: Signal<i8> = use_signal(|| 0);

  // TODO: Is using Arc<AtomicBool> more efficient than using a Signal<bool>?
  let blur_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  let focus_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  let update_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  // TODO: Using Signal seems cleaner than repeatedly cloning Arc<AtomicBool>
  // TODO: Revisit when Dioxus supports async closures
  let blur_flag_for_closure: Arc<AtomicBool> = blur_flag.clone();

  let focus_flag_for_closure: Arc<AtomicBool> = focus_flag.clone();

  let update_flag_for_closure: Arc<AtomicBool> = update_flag.clone();

  let looper_closure = move || {
    looper(
      blur_flag_for_closure.clone(),
      drift_signal,
      focus_flag_for_closure.clone(),
      update_flag_for_closure.clone(),
    )
  };

  use_future(looper_closure);

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
      onblur: move |_event| blur_flag.store(true, Ordering::SeqCst),
      onclick: move |event| on_click(event, &mut click_count),
      onfocus: move |_event| focus_flag.store(true, Ordering::SeqCst),
      onkeydown: move |_event| update_flag.store(true, Ordering::SeqCst),
      onwheel: move |event| on_wheel(&mut drift_signal, event),
      tabindex: 0,
      width: "470",
    }
    }
  }
}

async fn looper(
  blur_flag: Arc<AtomicBool>,
  mut drift_signal: Signal<i8>,
  focus_flag: Arc<AtomicBool>,
  update_flag: Arc<AtomicBool>,
) {
  let mut animator = Animator::new(CANVAS_ID, MESSAGE_START);

  let mut repaint = false;
  let mut running = true;
  let mut update = false;

  loop {
    if blur_flag.load(Ordering::SeqCst) {
      blur_flag.store(false, Ordering::SeqCst);

      animator.set_message(MESSAGE_START);

      running = true;
    }

    let delta: i8 = *drift_signal.read();

    if delta != 0 {
      drift_signal.set(0);

      animator.adjust_maximum_drift(delta);

      update = true;
    }

    if focus_flag.load(Ordering::SeqCst) {
      focus_flag.store(false, Ordering::SeqCst);

      animator.set_message(MESSAGE_CONTROLS);

      repaint = true;

      running = false;
    }

    if update_flag.load(Ordering::SeqCst) {
      update_flag.store(false, Ordering::SeqCst);

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

    // TODO: Can we replace this with a request_animation_frame?
    async_std::task::sleep(Duration::from_millis(17u64)).await;
  }
}

fn on_click(
  _event: Event<MouseData>,
  click_count: &mut i32,
) {
  *click_count += 1;

  info!("click count: {click_count:?}");
}

fn on_wheel(
  drift_signal: &mut Signal<i8>,
  event: Event<WheelData>,
) {
  let wheel_delta: WheelDelta = event.delta();

  let delta: f64 = match wheel_delta {
    Lines(lines_vector) => lines_vector.y,
    Pages(pages_vector) => pages_vector.y,
    Pixels(pixels_vector) => pixels_vector.y,
  };

  let drift_delta: i8 = delta.clamp(-128., 127.) as i8;

  drift_signal.set(drift_delta);
}
