use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicI8;

// TODO: Can we make this Copy?
pub struct Inputs {
  // TODO: Can we use Rc instead of Arc?
  pub blur: Arc<AtomicBool>,
  pub drift: Arc<AtomicI8>,
  pub focus: Arc<AtomicBool>,
  pub stop: Arc<AtomicBool>,
  pub update: Arc<AtomicBool>,
}
