use lit::*;
use solver::Solver;
use std::vec::Vec;
use rand::Rng;

mod lit;
mod solver;

extern crate rand;

fn main() {
//	let mut solver = Solver::new();
//	let x0 = Lit::new(solver.new_var());
//	let x1 = Lit::new(solver.new_var());
//	
//	solver.add_clause_from_lits(vec![x0, x0]).unwrap();
//	solver.add_clause_from_lits(vec![!x0, x1]).unwrap();
//	solver.add_clause_from_lits(vec![!x0, !x1]).unwrap();
//	println!("{}", solver);	
//	solver.simplify();
//	println!("{}", solver);	
//	println!("{}", solver.solve());
//	solver.print_model();
	random_test();
}

fn random_test() {
	let mut solver = Solver::new();
	{
		let var_n = 1000;		// number of variables
		let clause_ms = 40.;	// max size of each clause
		let clause_mn = 40000.;	// number of clauses
		let assign_mn = 10.;	// number of assignments (unit clauses)
		
		println!("Random Test: \n\tVar num: {}\n\tClause num: {}\n\tMax clause size: {}\n\tAssignment num: {}\n", var_n, clause_mn, clause_ms as usize, assign_mn as usize); 
		
		let mut lits = Vec::<Lit>::new();
		
		for i in solver.create_vars(var_n) {
			lits.push(Lit::new(i));
		}
	
		println!("Generating CNF...");	
		random_clauses(&mut solver, lits, var_n as f32, clause_ms, clause_mn, assign_mn);
	}
	
	let clauses = solver.get_oringin_clauses();
	
	println!("Solving...\n");
	
	let sat = solver.solve();
	solver.print_model();
	
	let model = solver.get_model();
	println!("Match: {}", if sat {verify(clauses, model)}else {true});
	solver.reset();
}

fn verify(clauses: Vec<Clause>, model: Vec<LitValue>) -> bool {
	for i in 0..clauses.len() {
		let lits = &clauses[i].get_all_lits();
		let mut result = false;
		for j in 0..lits.len() {
			let lit:Lit = lits[j];
			if lit.get_value() == model[lit.var_num()] {
				result = true;
				break;
			}
		}
		if !result {
			return false;
		}
	}
	true
}

fn random_clauses(solver: &mut Solver, x: Vec<Lit>, lit_n: f32, clause_ms: f32, clause_mn: f32, assign_mn: f32) {
	let mut rng = rand::thread_rng();
	
	let assign_n = (rng.next_f32() * assign_mn + 1.).floor() as usize;
	for _ in 0..assign_n {
		let lit_num = (rng.next_f32() * lit_n).floor() as usize;
		if rng.gen() {
			solver.add_clause_from_lits(vec![x[lit_num]]).unwrap();
		}else {
			solver.add_clause_from_lits(vec![!x[lit_num]]).unwrap();
		}
	}

	let clause_n = (rng.next_f32() * clause_mn + 1.).floor() as usize;
	for _ in 0..clause_n {
		let mut clause = Clause::new();
		let clause_s = (rng.next_f32() * clause_ms + 2.).floor() as usize;
		for _ in 0.. clause_s {
			let lit_num = (rng.next_f32() * lit_n).floor() as usize;
			if rng.gen() {
				clause.push(x[lit_num]);
			}else {
				clause.push(!x[lit_num]);
			}
		}
//		println!("{}", clause);
		solver.add_clause(clause).unwrap();
	}	
//	println!("{}", solver);
}