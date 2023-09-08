use dioxus::prelude::*;
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

struct Color {
  blue: u8,
  green: u8,
  red: u8,
}

impl Color {
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
pub fn Home(cx: Scope) -> Element {
  let click_count: &UseState<i32> = use_state(cx, || 0);
  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/useeffect.rs
  use_effect(cx, (), |()| async {
    let mut color: Color = generate_random_color();
    loop {
      let fill_style: JsValue = color.to_fill_style();
      paint_background(&fill_style);
      async_std::task::sleep(Duration::from_millis(17u64)).await;
      drift_color(&mut color);
    }
  });
  render! {
    div {
      class: "box",
    h1 {
      class: "app-home",
      "CroftSoft Dioxus Prototype"
    }
    p {
      class: "app-home",
      "This is the home page."
    }
    canvas {
      background_color: "black",
      cursor: "crosshair",
      height: "600",
      id: CANVAS_ID,
      // https://docs.rs/dioxus/latest/dioxus/events/index.html
      onclick: move |event| on_click(event, click_count),
      onmouseenter: move |event| on_mouse_enter(event),
      onmouseout: move |event| on_mouse_out(event),
      onwheel: move |event| on_wheel(event),
      width: "600",
    }
    }
  }
}

fn drift_color(color: &mut Color) {
  color.blue = drift_primary_color(color.blue);
  color.green = drift_primary_color(color.green);
  color.red = drift_primary_color(color.red);
}

fn drift_primary_color(primary_color: u8) -> u8 {
  let range: RangeInclusive<i8> = -1..=1;
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

fn on_click(
  event: Event<MouseData>,
  mut click_count: &UseState<i32>,
) {
  log::info!("onclick Event: {event:?}");
  click_count += 1;
  let current_value = *click_count.current();
  log::info!("click count: {current_value:?}");
}

fn on_mouse_enter(event: Event<MouseData>) {
  log::info!("onmouseenter Event: {event:?}");
}

fn on_mouse_out(event: Event<MouseData>) {
  log::info!("onmouseout Event: {event:?}");
}

fn on_wheel(event: Event<WheelData>) {
  log::info!("onwheel Event: {event:?}");
}

fn paint_background(fill_style: &JsValue) {
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
  canvas_context.set_fill_style(&fill_style);
  canvas_context.fill_rect(0., 0., canvas_width, canvas_height);
}
