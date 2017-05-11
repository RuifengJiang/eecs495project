use std::ops::Not;
use std::fmt;
use std::cmp::*;
use lit::LitValue::*;

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var {
	num: usize
}

impl Var {
	pub fn new(num: usize) -> Self {
		Var {
			num: num,
		}
	}
	
	fn get_num(&self) -> usize {
		self.num
	}
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

#[derive (Debug, Copy, Clone, PartialEq, Eq)]
pub enum LitValue {LTrue, LFalse, LUndef}

impl LitValue {
	pub fn equals(self, other: LitValue) -> bool {
		match self {
			LTrue => match other {
				LTrue => true,
				LFalse => false,
				LUndef => true,
			},
			LFalse => match other {
				LTrue => false,
				LFalse => true,
				LUndef => true,
			},
			LUndef => true,
		}
	}
}

impl Not for LitValue {
	type Output = LitValue;
	
	fn not(self) -> LitValue {
		match self {
			LTrue => LFalse,
			LFalse => LTrue,
			LUndef => LUndef,
		}
	}
}

impl fmt::Display for LitValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			LTrue => write!(f, "T"),
			LFalse => write!(f, "F"),
			LUndef => write!(f, "X"),
		}
	} 
}

#[derive (Debug, Copy, Clone, PartialEq, Eq)]
pub struct Lit {
	var: Var,
	value: LitValue,
}

impl Lit {
	pub fn new(var: Var) -> Self {
		Lit {
			var: var,
			value: LTrue,
		}
	}
	
	pub fn from(num: usize, value: LitValue) -> Result<Self, String> {
		match value {
			LTrue | LFalse => Ok(Lit {
					var: Var::new(num),
					value: value,
				}),
			LUndef => Err("undefined value".to_string()),
		}
	}
	
	pub fn var_num(&self) -> usize {
		self.var.get_num()
	}
	
	pub fn get_value(&self) -> LitValue {
		self.value
	}
}

impl Not for Lit {
	type Output = Lit;

	fn not(self) -> Lit {
		Lit {
			var: self.var,
			value: !self.value,
		}
	}
}

impl PartialOrd for Lit {
	fn partial_cmp(&self, other: &Lit) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Lit {
	fn cmp(&self, other: &Lit) -> Ordering {
		self.var.cmp(&other.var)
	}
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	if self.value == LTrue {
	        write!(f, "{}", self.var)
    	}else {
	    	write!(f, "~{}", self.var)
    	}
    }
}

#[derive (Debug, Clone)]
pub struct Clause {
	vec_lit: Vec<Lit>,
	max_lit: Option<usize>,
}

impl Clause {
	pub fn new() -> Self {
		Clause {
			vec_lit: Vec::<Lit>::new(),
			max_lit: None,
		}
	}
	
	pub fn push(&mut self, lit: Lit) {
		match self.max_lit {
			Some(max_lit) => if max_lit < lit.var_num() {self.max_lit = Some(lit.var_num());},
			None => self.max_lit = Some(lit.var_num()), 
		};
		self.vec_lit.push(lit);
	}
	
	pub fn len(&self) -> usize {
		self.vec_lit.len()
	}
	
	pub fn get(&self, idx: usize) -> Lit {
		self.vec_lit[idx]
	}
	
	pub fn valid(&self, num: usize) -> bool {
		if let Some(lit) = self.max_lit {
			lit < num
		}else {
			false
		}
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(").unwrap();
		for i in 0..self.vec_lit.len() {
			if i != 0 {
				write!(f, "\\/").unwrap();
			}
			write!(f, "{}", self.vec_lit[i]).unwrap();
		} 
		write!(f, ")")
	}
}