use lit::*;
use solver::Solver;
use std::vec::Vec;
use rand::Rng;
use time::now;

mod lit;
mod solver;

extern crate rand;
extern crate time;

fn main() {
//	(9)/\(~9\/~9)/\(19\/9\/9)/\(~19)/\(~16)/\(~9)/\(16)/\(16)
//	let mut solver = Solver::new();
//	let x0 = Lit::new(solver.new_var());
//	let x1 = Lit::new(solver.new_var());
//	let x2 = Lit::new(solver.new_var());
//	
//	solver.add_clause_from_lits(vec![x0]).unwrap();
//	solver.add_clause_from_lits(vec![x1, !x0]).unwrap();
//	solver.add_clause_from_lits(vec![x1, !x2]).unwrap();
//	solver.add_clause_from_lits(vec![x1, x2]).unwrap();
//	solver.add_clause_from_lits(vec![!x2]).unwrap();
//	
//	println!("{}", solver);	
////	solver.simplify();
////	println!("{}", solver);	
//	println!("{}", solver.solve());
//	solver.print_model();
	println!("Correctness Test: ");
	random_correctness_test(1000);
	println!("\nSpeed Test: ");
	random_speed_test(10);
}

fn random_correctness_test(num: usize) {
	for i in 0..num {
		if i % 100 == 0 {
			println!("Correctness test num: {}", i);
		} 
		let mut solver = Solver::new();
		{
			let var_n = 100;		// number of variables
			let clause_ms = 5.;	// max size of each clause
			let clause_mn = 1000.;	// number of clauses
			let assign_mn = 0.;	// number of assignments (unit clauses)
			
			let mut lits = Vec::<Lit>::new();
			
			for i in solver.create_vars(var_n) {
				lits.push(Lit::new(i));
			}
		
			random_clauses(&mut solver, lits, var_n as f32, clause_ms, clause_mn, assign_mn, false);
		}
		
		let clauses = solver.get_oringin_clauses();
		
		let sat = solver.solve();
		if sat && !verify(clauses, solver.get_model()) {
			println!("Wrong Model");
			return;
		}
	}
	println!("Test Passed");
}

fn random_speed_test(num: usize) {
	for i in 0..num {
		let mut solver = Solver::new();
		{
			let var_n = 10000;		// number of variables
			let clause_ms =40.;	// max size of each clause
			let clause_mn = 100000.;	// number of clauses
			let assign_mn = 20.;	// number of assignments (unit clauses)
			
			println!("Random Test {}: \n\tVar num: {}\n\tMax clause num: {}\n\tMax clause size: {}\n\tMax assignment num: {}\n", i + 1, var_n, clause_mn, clause_ms as usize, assign_mn as usize); 
			
			let mut lits = Vec::<Lit>::new();
			
			for i in solver.create_vars(var_n) {
				lits.push(Lit::new(i));
			}
		
			println!("Generating CNF...\n");	
			random_clauses(&mut solver, lits, var_n as f32, clause_ms, clause_mn, assign_mn, true);
	//		println!("{}", solver);
		}
		
		solver.set_iter_num(1000);
		
		let clauses = solver.get_oringin_clauses();
		
		println!("Solving...\n");
		
		let start_time = now();
		let sat = solver.solve();
		let end_time = now();
		let duration = end_time - start_time;
		print!("Total time: {}.{} s\t", duration.num_seconds(), duration.num_milliseconds() - duration.num_seconds() * 1000);
		
//		solver.print_model();
		
		print!("SAT: {}\t", sat);
		println!("Result Match: {}\n\n======================================================\n", if sat {verify(clauses, solver.get_model())} else {true});
		solver.reset();
	}
}

fn verify(clauses: Vec<Clause>, model: &[LitValue]) -> bool {
	for clause in &clauses {
		let lits = clause.get_all_lits();
		let mut result = false;
		for j in lits {
			let lit = j.0;
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

fn random_clauses(solver: &mut Solver, x: Vec<Lit>, lit_n: f32, clause_ms: f32, clause_mn: f32, assign_mn: f32, print: bool) {
	let mut rng = rand::thread_rng();
	
	let assign_n = (rng.next_f32() * assign_mn).floor() as usize;
	for _ in 0..assign_n {
		let lit_num = (rng.next_f32() * lit_n).floor() as usize;
		if rng.gen() {
			solver.add_clause_from_lits(vec![x[lit_num]]).unwrap();
		}else {
			solver.add_clause_from_lits(vec![!x[lit_num]]).unwrap();
		}
	}

	let clause_n = (rng.next_f32() * clause_mn + 1. + clause_mn).floor() as usize / 2;
	let mut total_size = 0;
	for _ in 0..clause_n {
		let mut clause = Clause::new();
		let clause_s = (rng.next_f32() * clause_ms + 2.).floor() as usize;
		total_size += clause_s;
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
	if print {
		println!("\tClause num: {}\n\tTotal clause size: {}\n\tAssignment num: {}\n", clause_n, total_size, assign_n);
	}
//	println!("{}", solver);
}