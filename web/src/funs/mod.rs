use std::collections::HashMap;
use tera::{Result, Value};
pub mod article;
pub mod user;

pub type GlobalFn = Box<dyn Fn(&HashMap<String, Value>) -> Result<Value> + Sync + Send>;
