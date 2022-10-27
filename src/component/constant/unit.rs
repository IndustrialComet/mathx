#[derive(Clone)]
pub struct Base {
    name: String,
    exponent: i8,
}

impl PartialEq for Base {
	fn eq(&self, other: &Self) -> bool {
		return self.name == other.name && self.exponent == other.exponent;
	}
}

impl Eq for Base {}

#[derive(Clone)]
pub struct Unit {
    bases: Vec<Base>
}

impl Unit {
	pub fn parse() -> Option<Result<Self,&'static str>> {
		return None;
	}
    pub fn new() -> Self {
        return Self {
            bases: Vec::new(),
        }
    }
}

impl PartialEq for Unit {
	fn eq(&self, other: &Self) -> bool {
		return self.bases.into_iter().all(|base| {
			return other.bases.into_iter().find(|predicate| {
				return base.name == predicate.name;
			}).unwrap_or(Base {name: base.name,exponent: base.exponent + 1}).exponent == base.exponent;
		})
	}
}

impl Eq for Unit {}