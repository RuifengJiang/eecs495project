use lit::*;
use lit::LitValue::*;
use std::fmt;

#[derive (Debug)]
struct CNF {
	clauses: 	Vec<Clause>,	//vector of clauses
	sat: 		Vec<usize>,		//represent if the clause is satisfied or not. A clause is unsat implies vec_sat[ci] == 0
}

impl CNF {
	fn new() -> Self {
		CNF {
			clauses: 	Vec::<Clause>::new(),
			sat: 		Vec::<usize>::new(),		
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
	var: 		Vec<LitValue>,				//the assignment of each variable
	expected:	Vec<(LitValue, usize)>,		//expected value of variable during sovling process
	map: 		VarMap,						//saves the lists of position of each variable appear in CNF
	propagated: Vec<bool>,					//if the value of a variable is propagated through CNF
}

impl Model {
	fn new() -> Self {
		Model {
			var: 		Vec::<LitValue>::new(),
			expected:	Vec::<(LitValue, usize)>::new(),
			map: 		VarMap::new(),
			propagated: Vec::<bool>::new(),
		}
	}
	
	fn new_var(&mut self) {
		self.var.push(LUndef);
		self.expected.push((LUndef, 0));
		self.propagated.push(false);
		self.map.new_var();
	}
	
	fn len(&self) -> usize {
		self.var.len()
	}
}

#[derive (Debug)]
struct VarMap {
	lit_num: 			usize,
	true_clause_list: 	Vec<VarPosList>,		//list of clauses where the variable is true
	false_clause_list: 	Vec<VarPosList>,		//list of clauses where the variable is false
	cnt: 				Vec<usize>,
}

type VarPos = (usize, usize);
type VarPosList = Vec<VarPos>;

impl VarMap {
	fn new() -> Self {
		VarMap {
			lit_num: 0,
			true_clause_list: 	Vec::<VarPosList>::new(),
			false_clause_list: 	Vec::<VarPosList>::new(),
			cnt: 				Vec::<usize>::new(),
		}
	}
	
	//add a new variable
	fn new_var(&mut self) {
		self.lit_num += 1;
		self.true_clause_list.push(Vec::<VarPos>::new());
		self.false_clause_list.push(Vec::<VarPos>::new());
		self.cnt.push(0);
	}
	
	//add a new clause
	fn add_clause(&mut self, idx: usize, clause: &Clause) {
		let lits = clause.get_all_lits();
		for (i, tuple) in lits.iter().enumerate() {
			let lit = tuple.0;
			let var_num = lit.var_num();
			self.cnt[var_num] += 1;
			
			if lit.get_value() == LTrue {
				self.true_clause_list[var_num].push((idx, i));
			}else {
				self.false_clause_list[var_num].push((idx, i));
			}
		}
	}
	
	fn get_clauses_of(&self, var: usize, val: LitValue) -> &[VarPos] {
		if val == LTrue {
			&self.true_clause_list[var]
		}else {
			&self.false_clause_list[var]
		}
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
			cnf: 		CNF::new(),
			len: 		0,
			num_var: 	0,
			model: 		Model::new(),
			status: 	true,
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
						
						self.model.map.add_clause(self.cnf.len(), &clause);
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
				self.model.map.add_clause(self.cnf.len(), &clause);
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
	
	pub fn get_model(&self) -> &[LitValue] {
		&self.model.var
	}
	
	pub fn get_oringin_clauses(&self) -> Vec<Clause> {
		self.cnf.clauses.clone()
	}
	
	pub fn print_model(&self) {
		if self.status {
			for var in self.model.var.iter() {
				print!("{}", var);
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
					if !self.model.propagated[i] {
						propagated = true;
						let var = self.model.var[i];
						if var != LUndef {
							//propagate the value forward
							let empty = self.propagate(i, var, true);
							if empty {
								self.status = false;
								return false;
							}
						}
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
		for i in 0..self.cnf.len() {
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
	fn propagate(&mut self, var: usize, value: LitValue, forward: bool) -> bool {
		self.model.propagated[var] = forward;
		let sat_list;
		let unsat_list;
		let mut result = false;
		
		if value == LTrue {
			sat_list = self.model.map.get_clauses_of(var, LTrue);
			unsat_list = self.model.map.get_clauses_of(var, LFalse);
		}else {
			sat_list = self.model.map.get_clauses_of(var, LFalse);
			unsat_list = self.model.map.get_clauses_of(var, LTrue);
		}

		for j in sat_list {
			if forward {
				if self.cnf.sat[j.0] == 0 {
					self.len -= 1;
				}
				self.cnf.sat[j.0] += 1;
			}else {
				self.cnf.sat[j.0] -= 1;
				if self.cnf.sat[j.0] == 0 {
					self.len += 1;
				}
			}
		}
		for j in unsat_list {
			if forward {
				self.cnf.clauses[j.0].remove(j.1);
				//check if the clause is empty
				let len = self.cnf.clauses[j.0].len();
				if len == 0 {
					result = true;
				//check if the clause becomes an assignment	
				}else if len == 1 {
					//get the lit from the assignment
					let lit = self.cnf.clauses[j.0].get_first().unwrap();
					let var = lit.var_num();
					let value = lit.get_value();
					//add assignment count by one
					self.model.expected[var].1 += 1;
					
					//check if there is a conflict on the assignment
					if self.model.expected[var].0.equals(value) {
						self.model.expected[var].0 = value;
					}else {
						result = true;
					}
				}
			}else {
				//check if the clause is an assignment
				if self.cnf.clauses[j.0].len() == 1 {
					let lit = self.cnf.clauses[j.0].get_first().unwrap();
					let var = lit.var_num();
					self.model.expected[var].1 -= 1;
					
					//undo assignment
					if self.model.expected[var].1 == 0 {
						self.model.expected[var].0 = LUndef;
					}
				}
				
				self.cnf.clauses[j.0].restore(j.1);
			}
		}
		result
	}

	pub fn solve(&mut self) -> bool {
		if self.status {
			if !self.simplify() {
				return false;
			}
			let mut hist = Vec::<(Lit, Option<Lit>)>::new();	//history stack
			let mut cnt = 0;	//iteration count
			let mut next_lit = None;
			let mut front_pt = 0;
			loop {
				cnt += 1;
				if cnt % (self.num_var / 10) == 0 {
					println!("Iteration: {}", cnt);
				}
				
				//check if need to find a new var to propagate
				if next_lit == None {
					while self.model.propagated[front_pt] {
						front_pt += 1;
					}
					let mut next_var = front_pt;
					
					for i in next_var..self.model.len() {
						if !self.model.propagated[i] && (self.model.expected[i].1 != 0 || self.model.var[i] != LUndef) {
							next_var = i;
							break;
						}
					}
					
					//check if the next var only can be false
					if self.model.map.get_clauses_of(next_var, LTrue).len() == 0 || self.model.expected[next_var].0 == LFalse {
						let lit = Lit::create(next_var, LFalse);
						next_lit = Some((lit, None));
					//check if the next var	only can be true
					}else if self.model.map.get_clauses_of(next_var, LFalse).len() == 0  || self.model.expected[next_var].0 == LTrue {
						let lit = Lit::create(next_var, LTrue);
						next_lit = Some((lit, None));
					//the value can be either true or false	
					}else {
						let lit = Lit::create(next_var, LTrue);
						next_lit = Some((lit, Some(!lit)));
					}
				}

				let lit = next_lit.unwrap().0;
				let var = lit.var_num();
				let value;
				
				//check if is an assignment from original CNF
				if self.model.var[var] == LUndef {
					value = lit.get_value();
					self.model.var[var] = value;
					//add the propagation into history stack
					hist.push(next_lit.unwrap());
				}else {
					//use the value from the original CNF assignment
					value = self.model.var[var];
					next_lit = None;
				}
				
				//propagate the value and get if there is any empty clause
				let empty_clause = self.propagate(var, value, true);
				
				if empty_clause {
					//undo propagation based on history stack
					while let Some((lit, next)) = hist.pop() {
						let var = lit.var_num();
						let val = lit.get_value();
						
						if var < front_pt {
							front_pt -= var;
						}
						//undo propagation
						self.propagate(var, val, false);
						self.model.var[var] = LUndef;
						match next {
							//if the var can be another value, use that value for next iteration
							Some(lit) => {next_lit = Some((lit, None)); break;},
							//check if the history is empty
							None => if hist.len() == 0 {self.status = false;return false;},
						}
					}
				}else {
					//if length is 0, the CNF is sat
					if self.len == 0 {
						break;
					}
					next_lit = None;
				}
			}
			println!("Total iteration: {}", cnt);
		}
		self.status
	}
	
	//reset the solver to the state before solving and simplifying
	pub fn reset(&mut self) {
		for i in 0..self.model.len() {
			self.model.propagated[i] = false;
			self.model.expected[i] = (LUndef, 0);
			self.model.var[i] = LUndef;
		}
		for i in 0..self.cnf.len() {
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
		for (i, c) in self.cnf.clauses.iter().enumerate() {
			if self.cnf.sat[i] == 0 {
				if !first {
					write!(f, "/\\").unwrap();
				}
				first = false;
				write!(f, "{}", c).unwrap();
			}
		}
		write!(f, "")
	}
}