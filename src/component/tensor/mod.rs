use super::Context;
use super::Trait;
use super::Component;

pub struct Tensor {
    components: Vec<Component>,
}

impl Trait for Tensor {
    fn parse(raw: &str) -> Option<Result<Self,&'static str>> {
        return 
    }
	fn evaluate(&self,context: &Context) -> Result<Tensor,&'static str> {
		return self.components.into_iter().map(|component| {
			return component.evaluate(context).map(|tensor| {
				return tensor.components;
			})
		}).collect::<Result<Vec<Vec<Component>>,&'static str>>().map(|components| {
			return Self {
				components: components.into_iter().flatten().collect(),
			}
		});
	}
}