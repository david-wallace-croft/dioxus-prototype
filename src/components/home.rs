use dioxus::prelude::*;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

// https://docs.rs/dioxus-hooks/latest/dioxus_hooks/
// use dioxus::hooks::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/useeffect.rs
  use_effect(cx, (), |()| async {
    let window = window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let html_canvas_element = document
      .get_element_by_id("game-of-life-canvas")
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
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..=255);
    loop {
      let r: u8 = die.sample(&mut rng);
      let g: u8 = die.sample(&mut rng);
      let b: u8 = die.sample(&mut rng);
      log::info!("rgb({r}, {g}, {b})");
      let rgb: String = format!("rgb({r}, {g}, {b})");
      let fill_style: JsValue = JsValue::from_str(&rgb);
      canvas_context.set_fill_style(&fill_style);
      canvas_context.fill_rect(0., 0., canvas_width, canvas_height);
      async_std::task::sleep(Duration::from_millis(1_000)).await;
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
      height: "600",
      id: "game-of-life-canvas",
      style: "cursor: crosshair",
      width: "600",
    }
    }
  }
}
