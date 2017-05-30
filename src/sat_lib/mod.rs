use sat_lib::VarValue::*;

use std::fmt;
use std::collections::HashSet;
use std::ops::Not;
use std::cmp::Ordering;

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
///Used to represent variable used in CNF.
pub struct Var {
	///Index number of variables. 
	num: usize
}

impl Var {
	///Create a new variable using given index number.
	///
	///# Examples
	///
	/// ```
	///let v = Var::new(4);
	///
	///println!("{}", v); //4
	/// ```
	///
	pub fn new(num: usize) -> Self {
		Var {
			num: num,
		}
	}
	
	///Return index number of the variable.
	///
	///# Examples
	///
	/// ```
	///let v = Var::new(5);
	///
	///println!("{}", v.var_num); //5
	/// ```
	///
	pub fn get_num(&self) -> usize {
		self.num
	}
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, Hash)]
///Used to represent the value of variable.
///
///# Examples
///
///```
///
///println!("{}", VTrue);  //T
///println!("{}", VFalse); //F
///println!("{}", VUndef); //X
///```
pub enum VarValue {
	///true
	VTrue, 
	///false
	VFalse, 
	///either true or false
	VUndef,
}

impl VarValue {
	///Check if the values are equal.
	///
	///# Examples
	///
	///```
	///let t = VTrue;
	///
	///println!("{}", t.equals(VTrue));  //true
	///println!("{}", t.equals(VFalse)); //false
	///println!("{}", t.equals(VUndef)); //true
	///
	///```
	pub fn equals(self, other: VarValue) -> bool {
		match self {
			VTrue => match other {
				VTrue => true,
				VFalse => false,
				VUndef => true,
			},
			VFalse => match other {
				VTrue => false,
				VFalse => true,
				VUndef => true,
			},
			VUndef => true,
		}
	}
}

impl Not for VarValue {
	type Output = VarValue;
	
	fn not(self) -> VarValue {
		match self {
			VTrue => VFalse,
			VFalse => VTrue,
			VUndef => VUndef,
		}
	}
}

impl fmt::Display for VarValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			VTrue => write!(f, "T"),
			VFalse => write!(f, "F"),
			VUndef => write!(f, "X"),
		}
	} 
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, Hash)]
///Used to represent literals in CNF
pub struct Lit {
	///Corresponding variable
	var: 	Var,
	///The value of literal
	value: 	VarValue,
}

impl Lit {
	///Create a new literal
	///
	///# Examples
	///
	///```
	///
	///let v = Var::new(5);
	///let x0 = Lit::new(v);
	///
	///println!("x0 = {}, !x0 = {}", x0, !x0); //x0 = 5, !x0 = ~5
	///```
	pub fn new(var: Var) -> Self {
		Lit {
			var: 	var,
			value: 	VTrue,
		}
	}
	
	fn create(var_num: usize, val: bool) -> Self {
		Lit {
			var: 	Var::new(var_num),
			value: 	if val {VTrue} else {VFalse},
		}
	}
	
	///Get the index number of the variable
	///
	///# Examples
	///
	///```
	///let v = Var::new(5);
	///let x0 = Lit::new(v);
	///let x0_num = x0.var_num();
	///
	///println!("v_num = {}", v_num); //v_num = 5
	///```
	pub fn var_num(&self) -> usize {
		self.var.get_num()
	}
	
	///Get the value of the variable
	///
	///# Examples
	///
	///```
	///let v = Var::new(5);
	///let x0 = Lit::new(v);
	///
	///println!("x0 value = {}, !x0 value = {}", x0.get_value(), !x0.get_value()); // x0 = T, !x0 = F
	///```
	pub fn get_value(&self) -> VarValue {
		self.value
	}
	
	///Given a list of variables, create and return a list of responding literals
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let vars = solver.create_vars(100);
	///let x = create_lits(&vars);
	///
	///solver.add_clause_from_lits(vec![x[0], !x[2], x[80]]).unwrap();
	///solver.add_clause_from_lits(vec![!x[57], x[24]).unwrap();
	///
	///println!("{}", solver); //(0\/~2\/80)/\(~57\/24)
	///```
	pub fn create_lits(vars: &[Var]) -> Vec<Lit> {
		let mut lits = Vec::<Lit>::new();
		for i in vars {
			lits.push(Lit::new(*i));
		}
		lits
	}
}

impl Not for Lit {
	type Output = Lit;

	fn not(self) -> Lit {
		Lit {
			var: 	self.var,
			value: 	!self.value,
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
    	if self.value == VTrue {
	        write!(f, "{}", self.var)
    	}else {
	    	write!(f, "~{}", self.var)
    	}
    }
}

#[derive (Debug, Clone)]
///Used to represent clauses
pub struct Clause {
	///Literals in the clause
	vec_lit: 	Vec<(Lit, bool)>,
	///The max index number of literals in the clause
	max_lit: 	Option<usize>,
	///The number of literals, which are not logically removed, in the clause
	len: 		usize,
}

impl Clause {
	pub fn new() -> Self {
		Clause {
			vec_lit: 	Vec::<(Lit, bool)>::new(),
			max_lit: 	None,
			len: 		0,
		}
	}
	
	///Push a literal at the end of the clause
	///
	///# Examples
	///
	///```
	///let x0 = Lit::new(Var::new(0));
	///let x1 = Lit::new(Var::new(1));
	///
	///let mut c = Clause::new();
	///c.push(x[0]);
	///c.push(!x[1]);
	///
	///println!("{}", c); // (0\/~1)
	///```
	pub fn push(&mut self, lit: Lit) {
		match self.max_lit {
			Some(max_lit) => if max_lit < lit.var_num() {self.max_lit = Some(lit.var_num());},
			None => self.max_lit = Some(lit.var_num()), 
		};
		self.vec_lit.push((lit, false));
		self.len += 1;
	}
	
	///get the number of literals in the clause
	///
	///# Examples
	///
	///```
	///let x0 = Lit::new(Var::new(0));
	///let x1 = Lit::new(Var::new(1));
	///
	///let mut c = Clause::new();
	///c.push(x[0]);
	///c.push(!x[1]);
	///
	///println!("{}", c.len()); // 2
	///```
	pub fn len(&self) -> usize {
		self.len
	}
	
	///return all lits, including those are marked
	///
	///# Examples
	///
	///```
	///let x0 = Lit::new(Var::new(0));
	///let x1 = Lit::new(Var::new(1));
	///
	///let mut c = Clause::new();
	///c.push(x[0]);
	///c.push(!x[1]);
	///
	///println!("{}", c.get_all_lits()[1]); //~1
	///```
	pub fn get_all_lits(&self) -> Vec<Lit> {
		let mut v = Vec::<Lit>::new();
		
		for i in &self.vec_lit {
			if !i.1 {
				v.push(i.0);
			}
		}
		
		v
	}
	
	//get first lit that is not marked
	fn get_first(&self) -> Option<Lit> {
		if self.len > 0{
			let mut i = 0;
			while self.vec_lit[i].1 {
				i += 1;
			}
			Some(self.vec_lit[i].0)
		}else {
			None
		}
	}
	
	//logically remove one lit
	fn remove(&mut self, idx: usize) {
		if !self.vec_lit[idx].1 {
			self.vec_lit[idx].1 = true;
			self.len -= 1;
		}
	}
	
	//restore the removed lit
	fn restore(&mut self, idx: usize) {
		if self.vec_lit[idx].1 {
			self.vec_lit[idx].1 = false;
			self.len += 1;
		}
	}
	
	//restore all lits, including not removed ones
	fn restore_all(&mut self) {
		self.len = self.vec_lit.len();
		for i in 0..self.len {
			self.vec_lit[i].1 = false;
		}
	}
	
	//check if this clause is a valid clause, i.e. all lits are valid in the solver
	fn get_max(&self) -> Option<usize> {
		if let Some(var_num) = self.max_lit {
			Some(var_num)
		}else {
			None
		}
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(").unwrap();
		let mut first = true;
			
		for i in 0..self.vec_lit.len() {
			if !self.vec_lit[i].1 {
				if !first {
					write!(f, "\\/").unwrap();
				}
				first = false;
				write!(f, "{}", self.vec_lit[i].0).unwrap();
			}
		} 
		write!(f, ")")
	}
}

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
	var: 		Vec<VarValue>,				//the assignment of each variable
	expected:	Vec<(VarValue, usize)>,		//expected value of variable during sovling process
	map: 		VarMap,						//saves the lists of position of each variable appear in CNF
	propagated: Vec<bool>,					//if the value of a variable is propagated through CNF
}

impl Model {
	fn new() -> Self {
		Model {
			var: 		Vec::<VarValue>::new(),
			expected:	Vec::<(VarValue, usize)>::new(),
			map: 		VarMap::new(),
			propagated: Vec::<bool>::new(),
		}
	}
	
	fn new_var(&mut self) {
		self.var.push(VUndef);
		self.expected.push((VUndef, 0));
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
		for (i, lit) in lits.iter().enumerate() {
			let var_num = lit.var_num();
			self.cnt[var_num] += 1;
			
			if lit.get_value() == VTrue {
				self.true_clause_list[var_num].push((idx, i));
			}else {
				self.false_clause_list[var_num].push((idx, i));
			}
		}
	}
	
	fn get_clauses_of(&self, var: usize, val: VarValue) -> &[VarPos] {
		if val == VTrue {
			&self.true_clause_list[var]
		}else {
			&self.false_clause_list[var]
		}
	}
}

#[derive (Debug)]
///SAT Solver
pub struct Solver {
	cnf: 		CNF,	//CNF  
	len: 		usize,
	num_var: 	usize,	//number of variables
	model: 		Model,
	status: 	bool,	//if the model is UNSAT or not. status == false implies the CNF is UNSAT.
	iter_num:	usize,
}

impl Solver {
	///Create a new SAT Solver
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let vars = solver.create_vars(100);
	///let x = create_lits(&vars);
	///
	///solver.add_clause_from_lits(vec![x[0], !x[2], x[80]]).unwrap();
	///println!("{}", solver); //(0\/~2\/80)
	///
	///solver.add_clause_from_lits(vec![!x[57], x[24]).unwrap();
	///println!("{}", solver); //(0\/~2\/80)/\(~57\/24)
	///```
	pub fn new() -> Self {
		Solver {
			cnf: 		CNF::new(),
			len: 		0,
			num_var: 	0,
			model: 		Model::new(),
			status: 	true,
			iter_num:	0,
		}
	}
	
	///Create multiple variables
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let vars = solver.create_vars(100);
	///let x = create_lits(&vars);
	///
	///solver.add_clause_from_lits(vec![x[0], !x[2], x[80]]).unwrap();
	///println!("{}", solver); //(0\/~2\/80)
	///
	///solver.add_clause_from_lits(vec![!x[57], x[24]).unwrap();
	///println!("{}", solver); //(0\/~2\/80)/\(~57\/24)
	///```
	pub fn create_vars(&mut self, num: usize) -> Vec<Var> {
		let mut vars = Vec::<Var>::new();
		
		for _ in 0..num {
			vars.push(self.new_var());
		}
		
		vars
	}
	
	//Create a new variable
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///solver.add_clause_from_lits(vec![x0]).unwrap();
	///solver.add_clause_from_lits(vec![!x1]).unwrap();
	///println!("{}", solver); //(0)/\(~1)
	///```
	pub fn new_var(&mut self) -> Var {
		let num = self.num_var;
		self.model.new_var();
		self.num_var += 1;
		Var::new(num)
	}
	
	///Set the frequency of printing iteration number. If num is 0, solver will not print iterations number.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///solver.set_iter_print_freq(1000); //print iteration number of every 1000 iterations.
	///```
	pub fn set_iter_print_freq(&mut self, num: usize) {
		self.iter_num = num;
	}
	
	///Add one clause into the solver. Return the solver is still satisfiable or not.
	///False means USNAT.  
	///
	///# Error
	///
	///This function will return error if the solver is already unsat.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///let mut c0 = Clause::new();
	///c0.push(x0);
	///c0.push(!x1);
	///
	///let mut c1 = Clause::new();
	///c1.push(!x0);
	///
	///solver.add_clause(c0).unwrap();
	///solver.add_clause(c1).unwrap();
	///println!("{}", solver); //(0\/~1)/\(~0)
	///```
	pub fn add_clause(&mut self, clause: Clause) -> Result<bool, String> {
		if self.status {
			if clause.len() == 0 {
				self.cnf.add_clause(clause);
				self.status = false;
				return Ok(self.status);
			}
			if let Some(var_num) = clause.get_max() {
				while var_num >= self.num_var {
					self.new_var();
				}
			}
			//if the clause is an assignment
			if clause.len() == 1 {
				let lit = clause.get_first().unwrap();
				//if this assignment is conflict to others
				if lit.get_value().equals(self.model.var[lit.var_num()]) {
					//if such an assignment has been performed before
					if self.model.var[lit.var_num()] == VUndef {
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
	
	///Create a clause from a list of literals and add into the solver. Return the solver is still satisfiable or not.
	///False means USNAT.  
	///
	///# Error
	///
	///This function will return error if the solver is already unsat.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///solver.add_clause_from_lits(vec![x0]).unwrap();
	///solver.add_clause_from_lits(vec![!x1]).unwrap();
	///println!("{}", solver); //(0)/\(~1)
	///```
	pub fn add_clause_from_lits(&mut self, lits: Vec<Lit>) -> Result<bool, String> {
		let mut c = Clause::new();
		for i in lits {
			c.push(i);
		}
		self.add_clause(c)
	}
	
	///Return the model (value of variables) of CNF.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///let mut c0 = Clause::new();
	///c0.push(x0);
	///c0.push(!x1);
	///
	///let mut c1 = Clause::new();
	///c1.push(!x0);
	///
	///solver.add_clause(c0).unwrap();
	///solver.add_clause(c1).unwrap();
	///
	///solver.solve();
	///
	///let model = solver.get_model();
	///println!("{}", model[0]); //F
	///println!("{}", model[1]); //F
	///```
	pub fn get_model(&self) -> &[VarValue] {
		&self.model.var
	}
	
	///Return a list of original clauses.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///let mut c0 = Clause::new();
	///c0.push(x0);
	///c0.push(!x1);
	///
	///let mut c1 = Clause::new();
	///c1.push(!x0);
	///
	///solver.add_clause(c0).unwrap();
	///solver.add_clause(c1).unwrap();
	///
	///solver.siimplify();
	///
	///let clauses = solver.get_oringin_clauses();
	///println!("{}", clauses[0]); //(0\/~1)
	///println!("{}", clauses[1]); //(~0)
	///```
	pub fn get_oringin_clauses(&self) -> Vec<Clause> {
		self.cnf.clauses.clone()
	}
	
	///Print the model (value of variables) of CNF.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///let mut c0 = Clause::new();
	///c0.push(x0);
	///c0.push(!x1);
	///
	///let mut c1 = Clause::new();
	///c1.push(!x0);
	///
	///solver.add_clause(c0).unwrap();
	///solver.add_clause(c1).unwrap();
	///
	///solver.solve();
	///solve.print_model(); //FF
	///```
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
	
	///Simplify the CNF. Return if the CNF is still satisfiable.
	///False means UNSAT.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let v2 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///let x2 = Lit::new(v2);
	///
	///let mut c0 = Clause::new();
	///c0.push(x0);
	///c0.push(!x1);
	///c0.push(x2);
	///
	///let mut c1 = Clause::new();
	///c1.push(!x0);
	///
	///solver.add_clause(c0).unwrap();
	///solver.add_clause(c1).unwrap();
	///
	///println!("{}", solver); //(0\/~1\/2)/\(~0)
	///let sat = solver.siimplify();
	///if sat {
	///	println!("{}", solver); //(~1\/2)
	///}else {
	///	println!("UNSAT");
	///}
	///```
	pub fn simplify(&mut self) -> bool{
		if self.status {
			loop {
				let mut propagated = false;
				for i in 0..self.num_var {
					//if the currented var is true and not propagated yet
					if !self.model.propagated[i] {
						propagated = true;
						let var = self.model.var[i];
						if var != VUndef {
							//propagate the value forward
							let empty = self.propagate(i, var, true, None);
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
	fn propagate(&mut self, var: usize, value: VarValue, forward: bool, mut assignment_set: Option<&mut HashSet<usize>>) -> bool {
		self.model.propagated[var] = forward;
		let sat_list;
		let unsat_list;
		let mut result = false;
		
		if value == VTrue {
			sat_list = self.model.map.get_clauses_of(var, VTrue);
			unsat_list = self.model.map.get_clauses_of(var, VFalse);
		}else {
			sat_list = self.model.map.get_clauses_of(var, VFalse);
			unsat_list = self.model.map.get_clauses_of(var, VTrue);
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
				if self.cnf.sat[j.0] == 0 {
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
						
						if let Some(ref mut set) = assignment_set {
							set.insert(var);
						}
						
						//check if there is a conflict on the assignment
						if self.model.expected[var].0.equals(value) {
							self.model.expected[var].0 = value;
						}else {
							result = true;
						}
					}
				}
			}else {
				//check if the clause is an assignment
				if self.cnf.sat[j.0] == 0 && self.cnf.clauses[j.0].len() == 1 {
					let lit = self.cnf.clauses[j.0].get_first().unwrap();
					let var = lit.var_num();
					self.model.expected[var].1 -= 1;
					
					//undo assignment
					if self.model.expected[var].1 == 0 {
						self.model.expected[var].0 = VUndef;
						if let Some(ref mut set) = assignment_set {
							set.remove(&var);
						}
					}
				}
				
				self.cnf.clauses[j.0].restore(j.1);
			}
		}
		result
	}

	///Solve the CNF. Return if the CNF is still satisfiable.
	///False means UNSAT.
	///
	///# Examples
	///
	///```
	///let mut solver = Solver::new();
	///let v0 = solver.new_var();
	///let v1 = solver.new_var();
	///let x0 = Lit::new(v0);
	///let x1 = Lit::new(v1);
	///
	///let mut c0 = Clause::new();
	///c0.push(x0);
	///c0.push(!x1);
	///
	///let mut c1 = Clause::new();
	///c1.push(!x0);
	///
	///solver.add_clause(c0).unwrap();
	///solver.add_clause(c1).unwrap();
	///
	///let sat = solver.solve();
	///if sat {
	///	solve.print_model(); //FF
	///}
	///```
	pub fn solve(&mut self) -> bool {
		if self.status {
			if !self.simplify() {
				return false;
			}
			let mut hist = Vec::<(Lit, Option<Lit>)>::new();	//history stack
			let mut cnt = 0;	//iteration count
			let mut next_lit = None;
			let mut front_pt = 0;
			let mut assignment_set = HashSet::<usize>::new();
			
			loop {
				cnt += 1;
				if self.iter_num != 0 && cnt % self.iter_num == 0 {
					println!("Iteration: {}", cnt);
				}
				
				//check if need to find a new var to propagate
				if next_lit == None {
					while self.model.propagated[front_pt] {
						front_pt += 1;
					}
					let mut next_var = front_pt;
					
					if !assignment_set.is_empty() {
						for i in assignment_set.iter() {
							next_var = *i;
							break;
						}
						assignment_set.remove(&next_var);
					}
					
					//check if the next var only can be false
					if self.model.map.get_clauses_of(next_var, VTrue).len() == 0 || self.model.expected[next_var].0 == VFalse {
						let lit = Lit::create(next_var, false);
						next_lit = Some((lit, None));
					//check if the next var	only can be true
					}else if self.model.map.get_clauses_of(next_var, VFalse).len() == 0  || self.model.expected[next_var].0 == VTrue {
						let lit = Lit::create(next_var, true);
						next_lit = Some((lit, None));
					//the value can be either true or false	
					}else {
						let lit = Lit::create(next_var, true);
						next_lit = Some((lit, Some(!lit)));
					}
				}

				let lit = next_lit.unwrap().0;
				let var = lit.var_num();
				let value;
				
				//check if is an assignment from original CNF
				if self.model.var[var] == VUndef {
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
				let empty_clause = self.propagate(var, value, true, Some(&mut assignment_set));
				
				if empty_clause {
					//undo propagation based on history stack
					while let Some((lit, next)) = hist.pop() {
						let var = lit.var_num();
						let val = lit.get_value();
						
						if var < front_pt {
							front_pt = var;
						}
						//undo propagation
						self.propagate(var, val, false, Some(&mut assignment_set));
						self.model.var[var] = VUndef;
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
			if self.iter_num != 0 {
				println!("\nTotal iteration: {}", cnt);
			}
		}
		self.status
	}
	
	///Reset the solver to the state before solving and simplifying
	pub fn reset(&mut self) {
		for i in 0..self.model.len() {
			self.model.propagated[i] = false;
			self.model.expected[i] = (VUndef, 0);
			self.model.var[i] = VUndef;
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