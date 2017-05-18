use lit::*;
use lit::LitValue::*;
use std::collections::HashSet;
use std::fmt;

#[derive (Debug)]
struct CNF {
	clauses: 	Vec<Clause>,	//vector of clauses
	sat: 		Vec<usize>,		//represent if the clause is satisfied or not. A clause is unsat implies vec_sat[ci] == 0
}

impl CNF {
	fn new() -> Self {
		CNF {
			clauses: Vec::<Clause>::new(),
			sat: Vec::<usize>::new(),		
		}
	}
	
	fn add_clause(&mut self, clause: Clause) {
		self.clauses.push(clause);
		self.sat.push(0);
	}
	
	fn len(&self) -> usize {
		self.clauses.len()
	}
}

#[derive (Debug)]
struct Model {
	var: 		Vec<LitValue>,	//the assignment of each variable
	map: 		VarMap,			//saves the lists of position of each variable appear in CNF
	propagated: Vec<bool>,		//if the value of a variable is propagated through CNF
}

impl Model {
	fn new() -> Self {
		Model {
			var: Vec::<LitValue>::new(),
			map: VarMap::new(),
			propagated: Vec::<bool>::new(),
		}
	}
	
	fn new_var(&mut self) {
		self.var.push(LUndef);
		self.propagated.push(false);
		self.map.new_var();
	}
	
	fn len(&self) -> usize {
		self.var.len()
	}
}

#[derive (Debug)]
struct VarMap {
	lit_num: 	usize,
	vec_true: 	Vec<VarPosLit>,	//list of clauses where the variable is true
	vec_false: 	Vec<VarPosLit>,	//list of clauses where the variable is false
	cnt: 		Vec<usize>,
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
			let lit = lits[i].0;
			let var_num = lit.var_num();
			self.cnt[var_num] += 1;
			
			if lit.get_value() == LTrue {
				self.vec_true[var_num].push((idx, i));
			}else {
				self.vec_false[var_num].push((idx, i));
			}
		}
	}
	
	fn get_true_clauses_of(&self, lit: usize) -> &[VarPos] {
		&self.vec_true[lit]
	}
	
	fn get_false_clauses_of(&self, lit: usize) -> &[VarPos] {
		&self.vec_false[lit]
	}
	
	fn get_cnt(&self, lit: usize) -> usize {
		self.cnt[lit]
	}
}

#[derive (Debug)]
pub struct Solver {
	cnf: 		CNF,	//CNF  
	len: 		usize,
	num_var: 	usize,	//number of variables
	model: 		Model,
	status: 	bool,	//if the model is UNSAT or not. status == false implies the CNF is UNSAT.
}

impl Solver {
	pub fn new() -> Self {
		Solver {
			cnf: CNF::new(),
			len: 0,
			num_var: 0,
			model: Model::new(),
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
		self.model.new_var();
		self.num_var += 1;
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
				if lit.get_value().equals(self.model.var[lit.var_num()]) {
					//if such an assignment has been performed before
					if self.model.var[lit.var_num()] == LUndef {
						self.model.var[lit.var_num()] = lit.get_value();
						
						self.model.map.add_clause(self.len, &clause);
						self.len += 1;
						self.cnf.add_clause(clause);
					}
				}else {
					//conflict assignment
					self.cnf.add_clause(clause);
					self.status = false;
				}
			}else {
				//add a regular clause into the solver
				self.model.map.add_clause(self.len, &clause);
				self.len += 1;
				self.cnf.add_clause(clause);
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
		self.model.var.clone()
	}
	
	pub fn get_oringin_clauses(&self) -> Vec<Clause> {
		self.cnf.clauses.clone()
	}
	
	pub fn print_model(&self) {
		if self.status {
			for i in 0..self.model.len() {
				print!("{}", self.model.var[i]);
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
					if self.model.var[i] == LTrue && !self.model.propagated[i] {
						propagated = true;
						//propagate the value forward
						self.propagate(i, true, true);
					
					//if the currented var is false and not propagated yet
					}else if self.model.var[i] == LFalse && !self.model.propagated[i] {
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
			if self.cnf.sat[i] == 0 && self.cnf.clauses[i].len() == 1 {
				result = true;
				let lit = self.cnf.clauses[i].get_first().unwrap();
				self.model.var[lit.var_num()] = lit.get_value(); 
			}
		}
		result
	}
	
	//propagate the value throughout the CNF
	//forward: true means perform the propagation, false means undo the propagation
	fn propagate(&mut self, i: usize, value: bool, forward: bool) {
		self.model.propagated[i] = forward;
		let sat_list;
		let unsat_list;
		
		if value {
			sat_list = self.model.map.get_true_clauses_of(i);
			unsat_list = self.model.map.get_false_clauses_of(i);
		}else {
			sat_list = self.model.map.get_false_clauses_of(i);
			unsat_list = self.model.map.get_true_clauses_of(i);
		}
					
		for j in sat_list {
			if forward {
				self.cnf.sat[j.0] += 1;
			}else {
				self.cnf.sat[j.0] -= 1;
			}
		}
		for j in unsat_list {
			if forward {
				self.cnf.clauses[j.0].remove(j.1);
			}else {
				self.cnf.clauses[j.0].restore(j.1);
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
			if self.cnf.sat[i] == 0 {
				all_sat = false;
				
				//empty clause means unsat
				if self.cnf.clauses[i].len() == 0 {
					empty_clause = true;
					break;
				//check if it is unit clause
				}else if self.cnf.clauses[i].len() == 1 {
					assignments.push(self.cnf.clauses[i].get_first().unwrap());
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
					self.model.var[var_num] = var_value;
					self.propagate(var_num, var_value == LTrue, true);
				}
			}
			
			if !self.recur_solve() {
				//undo the propagation
				for lit in proped_lits {
					let var_num = lit.var_num();
					let var_value = lit.get_value();
					self.model.var[var_num] = LUndef;
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
			self.model.var[var_num] = LTrue;
			self.propagate(var_num, true, true);
			
			//if CNF is sat under the assignment
			if !self.recur_solve() {
				//undo the propagation
				self.propagate(var_num, true, false);
				
				//set the value to be false
				self.model.var[var_num] = LFalse;
				self.propagate(var_num, false, true);
				
				//if CNF is sat under the assignment
				if !self.recur_solve() {
					//undo the propagation
					self.model.var[var_num] = LUndef;
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
			if self.model.map.get_cnt(i) > max_cnt && !self.model.propagated[i] {
				max_cnt = self.model.map.get_cnt(i); 
				var_num = i;
			}
		}
		var_num
	}
	
	//reset the solver to the state before solving and simplifying
	pub fn reset(&mut self) {
		for i in 0..self.model.len() {
			self.model.propagated[i] = false;
			self.model.var[i] = LUndef;
		}
		for i in 0..self.len {
			self.cnf.sat[i] = 0;
			self.cnf.clauses[i].restore_all();
			if self.cnf.clauses[i].len() == 1 {
				let lit = self.cnf.clauses[i].get_first().unwrap();
				self.model.var[lit.var_num()] = lit.get_value();
			}
		}
	}
}

impl fmt::Display for Solver {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut first = true;
		for i in 0..self.cnf.len() {
			if self.cnf.sat[i] == 0 {
				if !first {
					write!(f, "/\\").unwrap();
				}
				first = false;
				write!(f, "{}", self.cnf.clauses[i]).unwrap();
			}
		}
		write!(f, "")
	}
}