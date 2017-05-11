use lit::*;
use solver::Solver;
use std::vec::Vec;
use rand::Rng;

mod lit;
mod solver;

extern crate rand;

fn main() {
//	rand_test(50, 150, 5);
	let mut solver = Solver::new();
	{
		let lit_n = 1000;
		let clause_ms = 20.;
		let clause_mn = 12000.;
		let assign_mn = 10.;
		
		let mut lits = Vec::<Lit>::new();
		
		for i in solver.create_vars(lit_n) {
			lits.push(Lit::new(i));
		}
		
		random_clauses(&mut solver, lits, lit_n as f32, clause_ms, clause_mn, assign_mn);
	}
	
	let clauses = solver.get_clauses();

//	println!("{}", solver);
//	
//	solver.simplify();
//	println!("{}", solver);
//	solver.print_model();
	
	println!("start");
	let sat = solver.solve();
	println!("end");
	solver.print_model();
	
	let model = solver.get_model();
	println!("Match: {}", if sat {verify(clauses, model)}else {true});
}

fn verify(clauses: Vec<Clause>, model: Vec<LitValue>) -> bool {
	for i in 0..clauses.len() {
		let clause = &clauses[i];
		let mut result = false;
		for j in 0..clause.len() {
			let lit:Lit = clause.get(j);
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