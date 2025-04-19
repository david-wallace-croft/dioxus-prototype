use ::com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdaterInputs;

#[derive(Default)]
pub struct FrameRaterUpdaterInput {
  pub update_time_millis: f64,
}

impl FrameRaterUpdaterInputs for FrameRaterUpdaterInput {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    None
  }

  fn get_reset_requested(&self) -> bool {
    false
  }

  fn get_time_to_update(&self) -> bool {
    true
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    None
  }

  fn get_update_time_millis(&self) -> f64 {
    self.update_time_millis
  }
}
