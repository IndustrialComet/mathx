use super::Context;
use super::Trait;
use super::Component;
use super::Tensor;

pub struct Expression {
	function: Function,
	tensors: Vec<Tensor>,
}

impl Trait for Expression {
	fn parse(raw: &str) -> Option<Result<Self,&'static str>> {
		//Find where abc mixes with a  matrix ( ,, , ,, ,, , , ;; ; ;; )
		return 
	}
	fn evaluate(&self,context: &Context) -> Result<Tensor,&'static str> {
		return context.functions.find()
	}
}