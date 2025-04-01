use super::color::Color;
use ::web_sys::wasm_bindgen::JsCast;
use ::web_sys::{
  CanvasRenderingContext2d, Document, HtmlCanvasElement, Window, window,
};

pub struct Animator {
  canvas_height: f64,
  canvas_rendering_context_2d: CanvasRenderingContext2d,
  canvas_width: f64,
  color: Color,
  delta_x: f64,
  delta_y: f64,
  maximum_drift: u8,
  message: &'static str,
  square_size: f64,
  x: f64,
  y: f64,
}

impl Animator {
  pub fn new(
    canvas_id: &str,
    message: &'static str,
  ) -> Self {
    let window: Window = window().expect("global window does not exists");

    let document: Document =
      window.document().expect("expecting a document on window");

    let html_canvas_element = document
      .get_element_by_id(canvas_id)
      .expect("expecting a canvas in the document")
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();

    let canvas_rendering_context_2d: CanvasRenderingContext2d =
      html_canvas_element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let canvas_height: f64 = html_canvas_element.height() as f64;

    let canvas_width: f64 = html_canvas_element.width() as f64;

    let color: Color = Color::random();

    let square_size: f64 =
      100.0_f64.min(canvas_width / 2.).min(canvas_height / 2.);

    let delta_x: f64 = 1.;

    let delta_y: f64 = 1.;

    let x: f64 = -delta_x;

    let y: f64 = -delta_y;

    let maximum_drift: u8 = 0;

    Self {
      canvas_height,
      canvas_rendering_context_2d,
      canvas_width,
      color,
      delta_x,
      delta_y,
      maximum_drift,
      message,
      square_size,
      x,
      y,
    }
  }

  pub fn paint(&self) {
    self.canvas_rendering_context_2d.set_fill_style_str("black");

    self.canvas_rendering_context_2d.fill_rect(
      0.,
      0.,
      self.canvas_width,
      self.canvas_height,
    );

    let fill_style: String = self.color.as_fill_style_string();

    self
      .canvas_rendering_context_2d
      .set_fill_style_str(&fill_style);

    self.canvas_rendering_context_2d.fill_rect(
      self.x,
      self.y,
      self.square_size,
      self.square_size,
    );

    self.canvas_rendering_context_2d.set_font("30px Verdana");

    self.canvas_rendering_context_2d.set_fill_style_str("white");

    let _ = self
      .canvas_rendering_context_2d
      .fill_text(self.message, 4., 30.);
  }

  pub fn adjust_maximum_drift(
    &mut self,
    delta: i8,
  ) {
    self.maximum_drift = self.maximum_drift.saturating_add_signed(delta);
  }

  pub fn set_message(
    &mut self,
    message: &'static str,
  ) {
    self.message = message;
  }

  pub fn update(&mut self) {
    if self.delta_x > 0. {
      if self.x + self.delta_x + self.square_size > self.canvas_width {
        self.delta_x = -self.delta_x;
      }
    } else if self.x + self.delta_x < 0. {
      self.delta_x = -self.delta_x;
    }

    self.x += self.delta_x;

    if self.delta_y > 0. {
      if self.y + self.delta_y + self.square_size > self.canvas_height {
        self.delta_y = -self.delta_y;
      }
    } else if self.y + self.delta_y < 0. {
      self.delta_y = -self.delta_y;
    }

    self.y += self.delta_y;

    if self.maximum_drift == 0 {
      self.maximum_drift = 1;
    }

    self.color.drift(self.maximum_drift);
  }
}
