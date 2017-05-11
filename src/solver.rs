use lit::*;
use lit::LitValue::*;
use std::fmt;

#[derive (Debug)]
pub struct Solver {
	clauses: Vec<Clause>,
	num_var: usize,
	model: Vec<LitValue>,
	status: bool
}

impl Solver {
	pub fn new() -> Self {
		Solver {
			clauses: Vec::<Clause>::new(),
			num_var: 0,
			model: Vec::<LitValue>::new(),
			status: true,
		}
	}
	
	pub fn create_vars(&mut self, num: usize) -> Vec<Var> {
		let mut vars = Vec::<Var>::new();
		
		for _ in 0..num {
			vars.push(self.new_var());
		}
		
		vars
	}
	
	pub fn new_var(&mut self) -> Var {
		let num = self.num_var;
		self.model.push(LUndef);
		self.num_var += 1;
		Var::new(num)
	}
	
	pub fn add_clause(&mut self, clause: Clause) -> Result<bool, String> {
		if self.status {
			if !clause.valid(self.num_var) {
				return Err("unknown Lit".to_string());
			}
			if clause.len() == 1 {
				let lit = clause.get(0);
				if lit.get_value().equals(self.model[lit.var_num()]) {
					if self.model[lit.var_num()] == LUndef {
						self.model[lit.var_num()] = lit.get_value();
						self.clauses.push(clause);
					}
				}else {
					self.clauses.push(clause);
					self.status = false;
				}
			}else {
				self.clauses.push(clause);
			}
		}
		Ok(self.status)
	}
	
	pub fn add_clause_from_lits(&mut self, lits: Vec<Lit>) -> Result<bool, String> {
		let mut c = Clause::new();
		for i in lits {
			c.push(i);
		}
		self.add_clause(c)
	}
	
	pub fn get_model(&self) -> Vec<LitValue> {
		self.model.clone()
	}
	
	pub fn get_clauses(&self) -> Vec<Clause> {
		self.clauses.clone()
	}
	
	pub fn print_model(&self) {
		if self.status {
			for i in 0..self.model.len() {
				print!("{}", self.model[i]);
			}
			println!();
		}else {
			println!("UNSAT");
		}
	}
	
	pub fn simplify(&mut self) -> bool{
		if self.status {
			match simplify(&self.clauses, &mut self.model) {
				Some(clauses) => self.clauses = clauses,
				None => {self.clauses.clear(); self.status = false},
			} 
		}
		self.status
	}
	
	pub fn solve(&mut self) -> bool {
		self.simplify();
		self.status = solve(&self.clauses, &mut self.model);
		self.status
	}
}

fn solve(clauses: &Vec<Clause>, model: &mut Vec<LitValue>) -> bool {
	if clauses.len() == 0 {
		return true;
	}
	
	let var = clauses[0].get(0).var_num();
	model[var] = LTrue;
	
	if let Some(clauses) = simplify(&clauses, model) {
		if solve(&clauses, model) {
			return true;
		}
	}
	model[var] = LFalse;
	if let Some(clauses) = simplify(&clauses, model) {
		return solve(&clauses, model);
	}
	model[var] = LUndef;
	false
} 

fn simplify(clauses: &Vec<Clause>, model: &mut Vec<LitValue>) -> Option<Vec<Clause>> {
	let mut new_clauses = Vec::<Clause>::new();
	let mut done = true;
	for i in 0..clauses.len() {
		let ref clause = clauses[i];
		let mut sat = false;
		let mut new_clause = Clause::new();
		
		for j in 0.. clause.len() {
			let lit = clause.get(j);
			if model[lit.var_num()] == lit.get_value() {
				sat = true;
				break;
			}else if model[lit.var_num()] == LUndef {
				new_clause.push(lit);
			}
		}
		if !sat {
			if new_clause.len() == 0 {
				return None;
			}else if new_clause.len() == 1 {
				let lit = new_clause.get(0);
				model[lit.var_num()] = lit.get_value();
				done = false;
			}else {
				new_clauses.push(new_clause);
			}
		}
	}
	if done {
		Some(new_clauses)
	}else {
		simplify(&new_clauses, model)
	}
}

impl fmt::Display for Solver {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for i in 0..self.clauses.len() {
			if i != 0 {
				write!(f, "/\\").unwrap();
			}
			write!(f, "{}", self.clauses[i]).unwrap();
		}
		write!(f, "")
	}
}