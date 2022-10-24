use super::Context;
use super::Trait;
use super::Component;
use super::Tensor;

pub struct Variable {
    symbol: String,
}

impl Trait for Variable {
    fn parse(raw: &str) -> Option<Result<Self,&'static str>> {
        return Some(Ok(Self {symbol: raw.to_string()}));
    }
	fn evaluate(&self,context: &Context) -> Result<Tensor,&'static str> {
		return context.variables.find(self);
	}
}

impl PartialEq for Variable {
	fn eq(&self, other: &Self) -> bool {
		return self.symbol == other.symbol;
	}
}

impl Eq for Variable {}
