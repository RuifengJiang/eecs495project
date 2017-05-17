use lit::*;
use lit::LitValue::*;
use std::collections::HashSet;
use std::fmt;

#[derive (Debug)]
pub struct Solver {
	clauses: Vec<Clause>,	//CNF
	vec_sat: Vec<usize>,	//represent if the clause is satisfied or not. A clause is unsat implies vec_sat[ci] == 0  
	len: usize,
	num_var: usize,			//number of variables
	model: Vec<LitValue>,	//the assignment of each variable
	var_map: VarMap,		//saves the lists of position of each variable appear in CNF
	propagated: Vec<bool>,	//if the value of a variable is propagated through CNF
	status: bool,			//if the model is UNSAT or not. status == false implies the CNF is UNSAT.
}

#[derive (Debug)]
struct VarMap {
	lit_num: usize,
	vec_true: Vec<VarPosLit>,	//list of clauses where the variable is true
	vec_false: Vec<VarPosLit>,	//list of clauses where the variable is false
	cnt: Vec<usize>,
}

type VarPos = (usize, usize);
type VarPosLit = Vec<VarPos>;

impl VarMap {
	fn new() -> Self {
		VarMap {
			lit_num: 0,
			vec_true: Vec::<VarPosLit>::new(),
			vec_false: Vec::<VarPosLit>::new(),
			cnt: Vec::<usize>::new(),
		}
	}
	
	//add a new variable
	fn new_var(&mut self) {
		self.lit_num += 1;
		self.vec_true.push(Vec::<VarPos>::new());
		self.vec_false.push(Vec::<VarPos>::new());
		self.cnt.push(0);
	}
	
	//add a new clause
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
	
	fn get_true_clauses(&self, lit: usize) -> &[VarPos] {
		&self.vec_true[lit]
	}
	
	fn get_false_clauses(&self, lit: usize) -> &[VarPos] {
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
			var_map: VarMap::new(),			
			propagated: Vec::<bool>::new(),
			status: true,
		}
	}
	
	//create multiple variables
	pub fn create_vars(&mut self, num: usize) -> Vec<Var> {
		let mut vars = Vec::<Var>::new();
		
		for _ in 0..num {
			vars.push(self.new_var());
		}
		
		vars
	}
	
	//create a new variable
	pub fn new_var(&mut self) -> Var {
		let num = self.num_var;
		self.model.push(LUndef);
		self.propagated.push(false);
		self.num_var += 1;
		self.var_map.new_var();
		Var::new(num)
	}
	
	//add one clause into the solver
	pub fn add_clause(&mut self, clause: Clause) -> Result<bool, String> {
		if self.status {
			if !clause.valid(self.num_var) {
				return Err("unknown Lit".to_string());
			}
			//if the clause is an assignment
			if clause.len() == 1 {
				let lit = clause.get_first().unwrap();
				//if this assignment is conflict to others
				if lit.get_value().equals(self.model[lit.var_num()]) {
					//if such an assignment has been performed before
					if self.model[lit.var_num()] == LUndef {
						self.model[lit.var_num()] = lit.get_value();
						
						self.var_map.add_clause(self.len, &clause);
						self.len += 1;
						self.clauses.push(clause);
						self.vec_sat.push(0);
					}
				}else {
					//conflict assignment
					self.clauses.push(clause);
					self.vec_sat.push(0);
					self.status = false;
				}
			}else {
				//add a regular clause into the solver
				self.var_map.add_clause(self.len, &clause);
				self.len += 1;
				self.clauses.push(clause);
				self.vec_sat.push(0);
			}
			Ok(self.status)
		}else {
			Err("The model is already UNSAT".to_string())
		}
	}
	
	//create a clause from vector of literals and add into the solver
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
	
	//simplify the CNF
	pub fn simplify(&mut self) -> bool{
		if self.status {
			loop {
				let mut propagated = false;
				for i in 0..self.num_var {
					//if the currented var is true and not propagated yet
					if self.model[i] == LTrue && !self.propagated[i] {
						propagated = true;
						//propagate the value forward
						self.propagate(i, true, true);
					
					//if the currented var is false and not propagated yet
					}else if self.model[i] == LFalse && !self.propagated[i] {
						propagated = true;
						//propagate the value forward
						self.propagate(i, false, true);
					}
				}
				//if no propagation is performed or there is no more unit clause
				if !propagated || !self.perform_assignments() {
					break;
				}
			}
		}
		self.status
	}
	
	//check if there is any assignment (unit clause) in the CNF
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
	
	//propagate the value throughout the CNF
	//forward: true means perform the propagation, false means undo the propagation
	fn propagate(&mut self, i: usize, value: bool, forward: bool) {
		self.propagated[i] = forward;
		let sat_list;
		let unsat_list;
		
		if value {
			sat_list = self.var_map.get_true_clauses(i);
			unsat_list = self.var_map.get_false_clauses(i);
		}else {
			sat_list = self.var_map.get_false_clauses(i);
			unsat_list = self.var_map.get_true_clauses(i);
		}
					
		for j in 0..sat_list.len() {
			if forward {
				self.vec_sat[sat_list[j].0] += 1;
			}else {
				self.vec_sat[sat_list[j].0] -= 1;
			}
		}
		for j in 0..unsat_list.len() {
			if forward {
				self.clauses[unsat_list[j].0].remove(unsat_list[j].1);
			}else {
				self.clauses[unsat_list[j].0].restore(unsat_list[j].1);
			}
		}
	}

	pub fn solve(&mut self) -> bool {
		if self.status {
			self.simplify();
			self.status = self.recur_solve();
		}
		self.status
	}
	
	fn recur_solve(&mut self) -> bool {
		let mut empty_clause = false;
		let mut all_sat = true;
		let mut assignments = Vec::<Lit>::new();
		
		//check if CNF is sat, unsat, or undetermined and find if there is any assignment 
		for i in 0..self.len {
			if self.vec_sat[i] == 0 {
				all_sat = false;
				
				//empty clause means unsat
				if self.clauses[i].len() == 0 {
					empty_clause = true;
					break;
				//check if it is unit clause
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
			//list of lits that values are propagated
			let mut proped_lits = Vec::<Lit>::new(); 
			
			{
				let mut set = HashSet::<usize>::new();
				for lit in assignments {
					//check if this var has been propagated
					if set.contains(&lit.var_num()) {
						continue;
					}
					//add to history
					set.insert(lit.var_num());
					proped_lits.push(lit);
					
					//propagate the value forward
					let var_num = lit.var_num();
					let var_value = lit.get_value();
					self.model[var_num] = var_value;
					self.propagate(var_num, var_value == LTrue, true);
				}
			}
			
			if !self.recur_solve() {
				//undo the propagation
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
			//choose one var and assign a value
			let var_num = self.choose_var();
			
			//set the value to be true
			self.model[var_num] = LTrue;
			self.propagate(var_num, true, true);
			
			//if CNF is sat under the assignment
			if !self.recur_solve() {
				//undo the propagation
				self.propagate(var_num, true, false);
				
				//set the value to be false
				self.model[var_num] = LFalse;
				self.propagate(var_num, false, true);
				
				//if CNF is sat under the assignment
				if !self.recur_solve() {
					//undo the propagation
					self.model[var_num] = LUndef;
					self.propagate(var_num, false, false);
					return false;
				}
			}
			true
		}
	}
	
	//choose a variable based on how many times it appears
	fn choose_var(&self) -> usize {
		let mut max_cnt = 0;
		let mut var_num = 0;
		for i in 0..self.num_var {
			if self.var_map.get_cnt(i) > max_cnt && !self.propagated[i] {
				max_cnt = self.var_map.get_cnt(i); 
				var_num = i;
			}
		}
		var_num
	}
	
	
	//reset the solver to the state before solving and simplifying
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