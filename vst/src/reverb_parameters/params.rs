use std::sync::Arc;
mod bool_param;
pub use bool_param::BoolParam;
mod float_param;
pub use float_param::{FloatParam, FloatRange};
mod param_indexer;
pub use param_indexer::ParamIndexer;

pub trait Params {
  // type Plain: PartialEq;

  fn get_name(&self) -> &str;
  fn get_value(&self) -> f32;
  fn get_normalized_value(&self) -> f32;
  fn set_plain_value(&self, value: f32);
  fn get_display_value(&self, include_unit: bool) -> String;
  fn get_default_normalized_value(&self) -> f32;
  // fn with_value_to_string(self, callback: Arc<dyn Fn(Self::Plain) -> String + Send + Sync>)
  //   -> Self;
}
