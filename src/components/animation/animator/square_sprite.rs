use crate::components::animation::user_input::UserInput;

use super::super::color::Color;
use super::super::constants::VELOCITY_PIXELS_PER_MILLISECOND;
use ::web_sys::CanvasRenderingContext2d;

pub struct SquareSprite {
  boundary_x: f64,
  boundary_y: f64,
  color: Color,
  maximum_drift: u8,
  position: [f64; 2],
  square_size: f64,
  velocity: [f64; 2],
}

impl SquareSprite {
  pub fn new(
    boundary_x: f64,
    boundary_y: f64,
    square_size: f64,
  ) -> Self {
    Self {
      boundary_x,
      boundary_y,
      color: Color::random(),
      maximum_drift: 0,
      position: [0.; 2],
      square_size,
      velocity: [VELOCITY_PIXELS_PER_MILLISECOND; 2],
    }
  }

  pub fn paint(
    &self,
    canvas_rendering_context_2d: &CanvasRenderingContext2d,
  ) {
    let fill_style: String = self.color.as_fill_style_string();

    canvas_rendering_context_2d.set_fill_style_str(&fill_style);

    canvas_rendering_context_2d.fill_rect(
      self.position[0],
      self.position[1],
      self.square_size,
      self.square_size,
    );
  }

  pub fn update(
    &mut self,
    delta_time: f64,
    user_input: &UserInput,
  ) {
    self.update_color(user_input);

    self.update_position(delta_time);
  }

  fn update_color(
    &mut self,
    user_input: &UserInput,
  ) {
    let delta: i8 = user_input.drift;

    if delta != 0 {
      self.maximum_drift = self.maximum_drift.saturating_add_signed(delta);
    }

    if self.maximum_drift == 0 {
      self.maximum_drift = 1;
    }

    self.color.drift(self.maximum_drift);
  }

  fn update_position(
    &mut self,
    delta_time: f64,
  ) {
    self.update_position_bounce(self.boundary_x, delta_time, 0);

    self.update_position_bounce(self.boundary_y, delta_time, 1);
  }

  fn update_position_bounce(
    &mut self,
    boundary: f64,
    delta_time: f64,
    index: usize,
  ) {
    let delta_space: f64 = self.velocity[index] * delta_time;

    if delta_space > 0. {
      if self.position[index] + delta_space + self.square_size > boundary {
        self.velocity[index] = -self.velocity[index];

        self.position[index] = boundary - self.square_size;
      } else {
        self.position[index] += delta_space;
      }
    } else if self.position[index] + delta_space < 0. {
      self.velocity[index] = -self.velocity[index];

      self.position[index] = 0.;
    } else {
      self.position[index] += delta_space;
    }
  }
}
