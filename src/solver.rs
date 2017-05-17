use lit::*;
use lit::LitValue::*;
use std::collections::HashSet;
use std::fmt;

#[derive (Debug)]
pub struct Solver {
	clauses: Vec<Clause>,
	vec_sat: Vec<usize>,
	len: usize,
	num_var: usize,
	model: Vec<LitValue>,
	lit_map: LitMap,
	propagated: Vec<bool>,
	status: bool,
}

#[derive (Debug)]
struct LitMap {
	lit_num: usize,
	vec_true: Vec<LitPosLit>,
	vec_false: Vec<LitPosLit>,
	cnt: Vec<usize>,
}

type LitPos = (usize, usize);
type LitPosLit = Vec<LitPos>;

impl LitMap {
	fn new() -> Self {
		LitMap {
			lit_num: 0,
			vec_true: Vec::<LitPosLit>::new(),
			vec_false: Vec::<LitPosLit>::new(),
			cnt: Vec::<usize>::new(),
		}
	}
	
	fn new_lit(&mut self) {
		self.lit_num += 1;
		self.vec_true.push(Vec::<LitPos>::new());
		self.vec_false.push(Vec::<LitPos>::new());
		self.cnt.push(0);
	}
	
	fn add_clause(&mut self, idx: usize, clause: &Clause) {
		let lits = clause.get_all_lits();
		for i in 0..lits.len() {
			let lit = lits[i];
			let var_num = lit.var_num();
			self.cnt[var_num] += 1;
			if lit.get_value() == LTrue {
				self.vec_true[var_num].push((idx, i));
			}else {
				self.vec_false[var_num].push((idx, i));
			}
		}
	}
	
	fn get_true_clauses(&self, lit: usize) -> &[LitPos] {
		&self.vec_true[lit]
	}
	
	fn get_false_clauses(&self, lit: usize) -> &[LitPos] {
		&self.vec_false[lit]
	}
	
	fn get_cnt(&self, lit: usize) -> usize {
		self.cnt[lit]
	}
}

impl Solver {
	pub fn new() -> Self {
		Solver {
			clauses: Vec::<Clause>::new(),
			vec_sat: Vec::<usize>::new(),
			len: 0,
			num_var: 0,
			model: Vec::<LitValue>::new(),
			lit_map: LitMap::new(),			
			propagated: Vec::<bool>::new(),
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
		self.propagated.push(false);
		self.num_var += 1;
		self.lit_map.new_lit();
		Var::new(num)
	}
	
	pub fn add_clause(&mut self, clause: Clause) -> Result<bool, String> {
		if self.status {
			if !clause.valid(self.num_var) {
				return Err("unknown Lit".to_string());
			}
			if clause.len() == 1 {
				let lit = clause.get_first().unwrap();
				if lit.get_value().equals(self.model[lit.var_num()]) {
					if self.model[lit.var_num()] == LUndef {
						self.model[lit.var_num()] = lit.get_value();
						
						self.lit_map.add_clause(self.len, &clause);
						self.len += 1;
						self.clauses.push(clause);
						self.vec_sat.push(0);
					}
				}else {
					self.clauses.push(clause);
					self.vec_sat.push(0);
					self.status = false;
				}
			}else {
				self.lit_map.add_clause(self.len, &clause);
				self.len += 1;
				self.clauses.push(clause);
				self.vec_sat.push(0);
			}
			Ok(self.status)
		}else {
			Err("UNSAT".to_string())
		}
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
	
	pub fn get_oringin_clauses(&self) -> Vec<Clause> {
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
			loop {
				let mut propagated = false;
				for i in 0..self.num_var {
					if self.model[i] == LTrue && !self.propagated[i] {
						propagated = true;
						self.propagate(i, true, true);
					}else if self.model[i] == LFalse && !self.propagated[i] {
						propagated = true;
						self.propagate(i, false, true);
					}
				}
				if !propagated || !self.perform_assignments() {
					break;
				}
			}
		}
		self.status
	}
	
	fn perform_assignments(&mut self) -> bool {
		let mut result = false;
		for i in 0..self.len {
			if self.vec_sat[i] == 0 && self.clauses[i].len() == 1 {
				result = true;
				let lit = self.clauses[i].get_first().unwrap();
				self.model[lit.var_num()] = lit.get_value(); 
			}
		}
		result
	}
	
	fn propagate(&mut self, i: usize, value: bool, forward: bool) {
		self.propagated[i] = forward;
		
		if value {
			let vec_true = self.lit_map.get_true_clauses(i);
			let vec_false = self.lit_map.get_false_clauses(i);
						
			for j in 0..vec_true.len() {
				if forward {
					self.vec_sat[vec_true[j].0] += 1;
				}else {
					self.vec_sat[vec_true[j].0] -= 1;
				}
			}
			for j in 0..vec_false.len() {
				if forward {
					self.clauses[vec_false[j].0].remove(vec_false[j].1);
				}else {
					self.clauses[vec_false[j].0].restore(vec_false[j].1);
				}
			}
		}else {
			let vec_true = self.lit_map.get_true_clauses(i);
			let vec_false = self.lit_map.get_false_clauses(i);
						
			for j in 0..vec_false.len() {
				if forward {
					self.vec_sat[vec_false[j].0] += 1;
				}else {
					self.vec_sat[vec_false[j].0] -= 1;
				}
			}
						
			for j in 0..vec_true.len() {
				if forward {
					self.clauses[vec_true[j].0].remove(vec_true[j].1);
				}else {
					self.clauses[vec_true[j].0].restore(vec_true[j].1);
				}
			}
		}
	}

	pub fn solve(&mut self) -> bool {
		if self.status {
			self.simplify();
			self.status = self.try_solve(1);
		}
		self.status
	}
	
	fn try_solve(&mut self, level: usize) -> bool {
		let mut empty_clause = false;
		let mut all_sat = true;
		let mut assignments = Vec::<Lit>::new();
		
		for i in 0..self.len {
			if self.vec_sat[i] == 0 {
				all_sat = false;
				if self.clauses[i].len() == 0 {
					empty_clause = true;
					break;
				}else if self.clauses[i].len() == 1 {
					assignments.push(self.clauses[i].get_first().unwrap());
				}
			}
		}
			
		if empty_clause {
			false
		}else if all_sat {
			true
		}else if assignments.len() != 0 {
			let mut proped_lits = Vec::<Lit>::new(); 
			
			{
				let mut set = HashSet::<usize>::new();
				for lit in assignments {
					if set.contains(&lit.var_num()) {
						continue;
					}
					set.insert(lit.var_num());
					proped_lits.push(lit);
					let var_num = lit.var_num();
					let var_value = lit.get_value();
					self.model[var_num] = var_value;
					self.propagate(var_num, var_value == LTrue, true);
				}
			}
			
			if !self.try_solve(level + 1) {
				for lit in proped_lits {
					let var_num = lit.var_num();
					let var_value = lit.get_value();
					self.model[var_num] = LUndef;
					self.propagate(var_num, var_value == LTrue, false);
				}
				false
			}else {
				true
			}
		}else {
			let var_num = self.choose_var();
			
			self.model[var_num] = LTrue;
			self.propagate(var_num, true, true);
			if !self.try_solve(level + 1) {
				self.propagate(var_num, true, false);
				self.model[var_num] = LFalse;
				self.propagate(var_num, false, true);
				if !self.try_solve(level + 1) {
					self.model[var_num] = LUndef;
					self.propagate(var_num, false, false);
					return false;
				}
			}
			true
		}
	}
	
	fn choose_var(&self) -> usize {
		let mut max_cnt = 0;
		let mut var_num = 0;
		for i in 0..self.num_var {
			if self.lit_map.get_cnt(i) > max_cnt && !self.propagated[i] {
				max_cnt = self.lit_map.get_cnt(i); 
				var_num = i;
			}
		}
		var_num
	}
	
	pub fn reset(&mut self) {
		for i in 0..self.propagated.len() {
			self.propagated[i] = false;
			self.model[i] = LUndef;
		}
		for i in 0..self.len {
			self.vec_sat[i] = 0;
			self.clauses[i].restore_all();
			if self.clauses[i].len() == 1 {
				let lit = self.clauses[i].get_first().unwrap();
				self.model[lit.var_num()] = lit.get_value();
			}
		}
	}
}

impl fmt::Display for Solver {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut first = true;
		for i in 0..self.clauses.len() {
			if self.vec_sat[i] == 0 {
				if !first {
					write!(f, "/\\").unwrap();
				}
				first = false;
				write!(f, "{}", self.clauses[i]).unwrap();
			}
		}
		write!(f, "")
	}
}