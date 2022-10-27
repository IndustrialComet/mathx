mod context;

mod constant;
mod variable;
mod expression;
mod tensor;

use context::Context;

use constant::Constant;
use variable::Variable;
use expression::Expression;
use tensor::Tensor;

pub trait Trait where Self: Sized {
	fn parse(raw: &str) -> Option<Result<Self,&'static str>>;
	fn evaluate(&self,context: &Context) -> Result<Tensor,&'static str>;
}

pub enum Component {
	Constant(Constant),
	Variable(Variable),
	Expression(Expression),
	Tensor(Tensor),
}

impl Trait for Component {
	fn parse(raw: &str) -> Option<Result<Self,&'static str>> {
		return Tensor::parse(raw).map(|result| {result.map(|tensor| Component::Tensor(tensor))}).or_else(|| {
			return Expression::parse(raw).map(|result| {result.map(|expression| Component::Expression(expression))}).or_else(|| {
				return Constant::parse(raw).map(|result| {result.map(|constant| Component::Constant(constant))}).or_else(|| {
					return Variable::parse(raw).map(|result| {result.map(|variable| Component::Variable(variable))});
				});
			});
		});
	}
	fn evaluate(&self,context: &Context) -> Result<Tensor,&'static str> {
		return match &self {
			Self::Tensor(tensor) => tensor.evaluate(context),
			Self::Constant(constant) => constant.evaluate(context),
			Self::Variable(variable) => variable.evaluate(context),
			Self::Expression(expression) => expression.evaluate(context),
		}
	}
}

negate
invert