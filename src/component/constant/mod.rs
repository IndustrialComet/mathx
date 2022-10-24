use super::Context;
use super::Trait;
use super::Component;
use super::Tensor;

mod unit;

use unit::Unit;

pub struct Constant {
    value: f64,//THIS WILL CHANGE
    unit: Unit,
}

impl Trait for Constant {
    fn parse(raw: &str) -> Option<Result<Self,&'static str>> {
		return raw.parse::<f64>().map(|value| {
			return Ok(Self {
				value,
				unit: Unit::new(),
			});
		}).ok();
    }
	fn evaluate(&self,context: &Context) -> Result<Tensor,&'static str> {
		return Ok(Tensor {
			components: vec![Component::Constant(self.clone())],
		});
	}
}

impl PartialEq for Constant {
	fn eq(&self, other: &Self) -> bool {
		return self.value == other.value && self.unit == other.unit;
	}
}

impl Eq for Constant {}