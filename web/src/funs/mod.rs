
use std::collections::HashMap;
use tera::{Value, Result};
pub mod article;

pub type GlobalFn =  Box<dyn Fn(&HashMap<String, Value>) -> Result<Value> + Sync + Send>;
