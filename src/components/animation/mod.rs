use dioxus::html::geometry::WheelDelta;
use dioxus::prelude::*;
use dioxus_elements::geometry::WheelDelta::Lines;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use std::ops::RangeInclusive;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

// https://docs.rs/dioxus-hooks/latest/dioxus_hooks/
// use dioxus::hooks::*;

const CANVAS_ID: &str = "home-page-canvas";
const MESSAGE_CONTROLS: &str = "Hold a key or scroll the mouse wheel";
const MESSAGE_START: &str = "Click on or tab to the canvas";

struct Color {
  blue: u8,
  green: u8,
  red: u8,
}

impl Color {
  // TODO: Use From or Into
  fn to_fill_style(&self) -> JsValue {
    let Color {
      blue,
      green,
      red,
    } = self;
    let rgb: String = format!("rgb({red}, {green}, {blue})");
    // log::info!("{rgb}");
    JsValue::from_str(&rgb)
  }
}

#[allow(non_snake_case)]
pub fn Animation(cx: Scope) -> Element {
  let click_count_state: &UseState<i32> = use_state(cx, || 0);
  let color_state: &UseState<Color> = use_state(cx, generate_random_color);
  let message_state: &UseState<&str> = use_state(cx, || MESSAGE_START);
  let running_state: &UseState<bool> = use_state(cx, || true);
  let update_state: &UseState<bool> = use_state(cx, || true);
  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/useeffect.rs
  use_effect(cx, (), |()| {
    to_owned![color_state];
    to_owned![message_state];
    to_owned![running_state];
    to_owned![update_state];
    async move {
      loop {
        if *running_state.current() || *update_state.current() {
          update_state.set(false);
          color_state.set(drift_color(&color_state.current()));
          let fill_style: JsValue = color_state.current().to_fill_style();
          paint(&fill_style, *message_state.current());
        }
        async_std::task::sleep(Duration::from_millis(17u64)).await;
      }
    }
  });
  render! {
    div {
      class: "app-animation box",
    h1 {
      "Animation"
    }
    canvas {
      background_color: "black",
      cursor: "crosshair",
      // height: "600",
      id: CANVAS_ID,
      // https://docs.rs/dioxus/latest/dioxus/events/index.html
      onblur: move |event| on_blur(event, message_state, running_state),
      onclick: move |event| on_click(event, click_count_state, color_state, update_state),
      onfocus: move |event| on_focus(event, message_state, running_state, update_state),
      onkeydown: move |event| on_key_down(event, color_state, update_state),
      onmouseenter: on_mouse_enter,
      onmouseout: on_mouse_out,
      onwheel: move |event| on_wheel(event, color_state, update_state),
      // style: "overscroll-behavior: none",
      tabindex: 0,
      width: "600",
    }
    }
  }
}

fn drift_color(color: &Color) -> Color {
  Color {
    blue: drift_primary_color(color.blue),
    green: drift_primary_color(color.green),
    red: drift_primary_color(color.red),
  }
}

fn drift_primary_color(primary_color: u8) -> u8 {
  let range: RangeInclusive<i8> = -6..=6;
  let die: Uniform<i8> = Uniform::from(range);
  let mut rng: ThreadRng = rand::thread_rng();
  let delta: i8 = die.sample(&mut rng);
  primary_color.saturating_add_signed(delta)
}

fn generate_random_color() -> Color {
  let mut rng: ThreadRng = rand::thread_rng();
  let d256: Uniform<u8> = Uniform::from(0..=255);
  let red: u8 = d256.sample(&mut rng);
  let green: u8 = d256.sample(&mut rng);
  let blue: u8 = d256.sample(&mut rng);
  Color {
    blue,
    green,
    red,
  }
}

fn on_blur(
  _event: Event<FocusData>,
  message_state: &UseState<&str>,
  running_state: &UseState<bool>,
) {
  // log::info!("onblur Event: {event:?}");
  message_state.set(MESSAGE_START);
  running_state.set(true);
}

fn on_click(
  _event: Event<MouseData>,
  mut click_count_state: &UseState<i32>,
  _color_state: &UseState<Color>,
  _update_state: &UseState<bool>,
) {
  // log::info!("onclick Event: {event:?}");
  click_count_state += 1;
  let current_value = *click_count_state.current();
  log::info!("click count: {current_value:?}");
  // color_state.set(generate_random_color());
  // update_state.set(true);
}

fn on_focus(
  _event: Event<FocusData>,
  message_state: &UseState<&str>,
  running_state: &UseState<bool>,
  update_state: &UseState<bool>,
) {
  // log::info!("onfocus Event: {event:?}");
  message_state.set(MESSAGE_CONTROLS);
  running_state.set(false);
  update_state.set(true);
}

fn on_key_down(
  _event: Event<KeyboardData>,
  color_state: &UseState<Color>,
  update_state: &UseState<bool>,
) {
  // log::info!("onkeydown Event: {event:?}");
  color_state.set(drift_color(&color_state.current()));
  update_state.set(true);
}

fn on_mouse_enter(_event: Event<MouseData>) {
  // log::info!("onmouseenter Event: {event:?}");
}

fn on_mouse_out(_event: Event<MouseData>) {
  // log::info!("onmouseout Event: {event:?}");
}

fn on_wheel(
  event: Event<WheelData>,
  color_state: &UseState<Color>,
  update_state: &UseState<bool>,
) {
  // log::info!("onwheel Event: {event:?}");
  let wheel_delta: WheelDelta = event.delta();
  let Lines(lines_vector) = wheel_delta else {
    return;
  };
  let delta = lines_vector.y.clamp(-128., 127.) as i8;
  color_state.set(shift_color(&color_state.current(), delta));
  // event.stop_propagation();
  update_state.set(true);
}

fn paint(
  fill_style: &JsValue,
  message: &str,
) {
  let window = window().expect("global window does not exists");
  let document = window.document().expect("expecting a document on window");
  let html_canvas_element = document
    .get_element_by_id(CANVAS_ID)
    .expect("expecting a canvas in the document")
    .dyn_into::<HtmlCanvasElement>()
    .unwrap();
  let canvas_context = html_canvas_element
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap();
  let canvas_height: f64 = html_canvas_element.height() as f64;
  let canvas_width: f64 = html_canvas_element.width() as f64;
  canvas_context.set_fill_style(fill_style);
  canvas_context.fill_rect(0., 0., canvas_width, canvas_height);
  canvas_context.set_font("30px Verdana");
  canvas_context.set_fill_style(&JsValue::from_str("black"));
  let _ = canvas_context.fill_text(message, 4., 30.);
}

fn shift_color(
  color: &Color,
  delta: i8,
) -> Color {
  Color {
    blue: shift_primary_color(color.blue, delta),
    green: shift_primary_color(color.green, delta),
    red: shift_primary_color(color.red, delta),
  }
}

fn shift_primary_color(
  primary_color: u8,
  delta: i8,
) -> u8 {
  primary_color.saturating_add_signed(delta)
}
