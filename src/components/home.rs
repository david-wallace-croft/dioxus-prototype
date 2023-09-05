use dioxus::prelude::*;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

// https://docs.rs/dioxus-hooks/latest/dioxus_hooks/
// use dioxus::hooks::*;

const CANVAS_ID: &str = "home-page-canvas";

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/useeffect.rs
  use_effect(cx, (), |()| async {
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
    let mut rng: ThreadRng = rand::thread_rng();
    let d256: Uniform<u8> = Uniform::from(0..=255);
    let duration: f64 = 1.;
    let mut r: u8 = d256.sample(&mut rng);
    let mut g: u8 = d256.sample(&mut rng);
    let mut b: u8 = d256.sample(&mut rng);
    let d3: Uniform<u8> = Uniform::from(0..=2);
    loop {
      let rgb: String = format!("rgb({r}, {g}, {b})");
      // log::info!("{rgb}");
      let fill_style: JsValue = JsValue::from_str(&rgb);
      canvas_context.set_fill_style(&fill_style);
      canvas_context.fill_rect(0., 0., canvas_width, canvas_height);
      async_std::task::sleep(Duration::from_millis(duration as u64)).await;
      // duration *= 1.1;
      let delta = d3.sample(&mut rng);
      if delta == 0 {
        r = r.saturating_sub(1);
      } else if delta == 2 {
        r = r.saturating_add(1);
      }
      let delta = d3.sample(&mut rng);
      if delta == 0 {
        g = g.saturating_sub(1);
      } else if delta == 2 {
        g = g.saturating_add(1);
      }
      let delta = d3.sample(&mut rng);
      if delta == 0 {
        b = b.saturating_sub(1);
      } else if delta == 2 {
        b = b.saturating_add(1);
      }
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
      onblur: move |event| log::info!("onblur Event: {event:?}"),
      onclick: move |event| log::info!("onclick Event: {event:?}"),
      onfocus: move |event| log::info!("onfocus Event: {event:?}"),
      onfocusin: move |event| log::info!("onfocusin Event: {event:?}"),
      onfocusout: move |event| log::info!("onfocusout Event: {event:?}"),
      onkeydown: move |event| log::info!("onkeydown Event: {event:?}"),
      onkeypress: move |event| log::info!("onkeypress Event: {event:?}"),
      onkeyup: move |event| log::info!("onkeyup Event: {event:?}"),
      onwheel: move |event| log::info!("onwheel Event: {event:?}"),
      width: "600",
    }
    }
  }
}
