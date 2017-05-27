use sat::sat::*;
use std::vec::Vec;
use rand::Rng;
use time::now;

extern crate sat;
extern crate rand;
extern crate time;

pub fn test() {
	sat_test();

	println!("Correctness Test: ");
	random_correctness_test(1000);

	println!("\nEfficiency Test: ");
	random_efficiency_test(3);
}

fn sat_test() {
//	(1\/~6\/8)/\(2\/~8\/4)/\(9\/~4\/1)/\(8\/~4)/\(7\/5\/0)/\(~0\/8\/~5)/\(~9\/~1)/\(1\/6)/\
//	(8\/2)/\(2\/5\/~3)/\(~1\/9\/9)/\(1\/6\/~2)/\(7\/~4\/9)/\(~0\/~8)/\(2\/~2)/\(2\/7\/~0)/\(4\/6)
	let mut solver = Solver::new();
	let v = solver.create_vars(10);
	let x = Lit::create_lits(&v); 
	
	solver.add_clause_from_lits(vec![x[1], !x[6], x[8]]).unwrap();
	solver.add_clause_from_lits(vec![x[2], !x[8], x[4]]).unwrap();
	solver.add_clause_from_lits(vec![x[9], !x[4], x[1]]).unwrap();
	solver.add_clause_from_lits(vec![x[8], !x[4]]).unwrap();
	solver.add_clause_from_lits(vec![x[7], x[5], x[0]]).unwrap();
	solver.add_clause_from_lits(vec![!x[0], x[8], !x[5]]).unwrap();
	solver.add_clause_from_lits(vec![!x[9], !x[1]]).unwrap();
	solver.add_clause_from_lits(vec![x[1], x[6]]).unwrap();
	solver.add_clause_from_lits(vec![x[8], x[2]]).unwrap();
	solver.add_clause_from_lits(vec![x[2], x[5], !x[3]]).unwrap();
	solver.add_clause_from_lits(vec![!x[1], x[9], x[9]]).unwrap();
	solver.add_clause_from_lits(vec![x[1], x[6], !x[2]]).unwrap();
	solver.add_clause_from_lits(vec![x[7], !x[4], x[9]]).unwrap();
	solver.add_clause_from_lits(vec![!x[0], !x[8]]).unwrap();
	solver.add_clause_from_lits(vec![x[2], !x[2]]).unwrap();
	solver.add_clause_from_lits(vec![x[2], x[7], !x[0]]).unwrap();
	solver.add_clause_from_lits(vec![x[4], x[6]]).unwrap();
	solver.add_clause_from_lits(vec![]).unwrap();
	
	println!("{}", solver);	
//	solver.simplify();
//	println!("{}", solver);	
	println!("{}", solver.solve());
	solver.print_model();
}

fn random_correctness_test(num: usize) {
	let mut sat_case = 0;
	let mut unsat_case = 0;
	for i in 0..num {
		if i % 1000 == 0 {
			println!("Correctness test num: {}", i);
		} 
		let mut solver = Solver::new();
		let var_n = 10;		// number of variables
		{
			let clause_ms = 5.;	// max size of each clause
			let clause_mn = 50.;	// number of clauses
			let assign_mn = 0.;	// number of assignments (unit clauses)
			
			let mut lits = Vec::<Lit>::new();
			
			for i in solver.create_vars(var_n) {
				lits.push(Lit::new(i));
			}
		
			random_clauses(&mut solver, lits, var_n as f32, clause_ms, clause_mn, assign_mn, false);
		}
		
		let clauses = solver.get_oringin_clauses();
		
		let sat = solver.solve();
		
		if sat {
			sat_case += 1;
			if !verify(&clauses, solver.get_model()) {
				println!("Wrong Model");
				return;
			}
		}else {
			unsat_case += 1;
			if !verify_unsat(&clauses, var_n) {
				println!("CNF is sat");
				return;
			}
		}
	}
	println!("Num of sat: {}\tNum of unsat: {}", sat_case, unsat_case);
	println!("Test Passed");
}

fn random_efficiency_test(num: usize) {
	for i in 0..num {
		let mut solver = Solver::new();
		{
			let var_n = 10000;		// number of variables
			let clause_ms = 40.;	// max size of each clause
			let clause_mn = 100000.;	// number of clauses
			let assign_mn = 20.;	// number of assignments (unit clauses)
			
			solver.set_iter_num(var_n * 2);
			
			println!("Random Test {}: \n\tVar num: {}\n\tMax clause num: {}\n\tMax clause size: {}\n\tMax assignment num: {}\n", i + 1, var_n, clause_mn, clause_ms as usize, assign_mn as usize); 
			
			let mut lits = Vec::<Lit>::new();
			
			for i in solver.create_vars(var_n) {
				lits.push(Lit::new(i));
			}
		
			println!("Generating CNF...\n");	
			random_clauses(&mut solver, lits, var_n as f32, clause_ms, clause_mn, assign_mn, true);
	//		println!("{}", solver);
		}
		
		let clauses = solver.get_oringin_clauses();
		
		println!("Solving...\n");
		
		let start_time = now();
		let sat = solver.solve();
		let end_time = now();
		let duration = end_time - start_time;
		print!("Total time: {}.{} s\t", duration.num_seconds(), duration.num_milliseconds() - duration.num_seconds() * 1000);
		
//		solver.print_model();
		
		print!("SAT: {}\t", sat);
		println!("Result Match: {}\n\n======================================================\n", if sat {verify(&clauses, solver.get_model())} else {true});
		solver.reset();
	}
}

fn verify_unsat(clauses: &[Clause], var_num: usize) -> bool {
	for i in 0..1024 {
		let mut model = Vec::<VarValue>::new();
		for j in 0..var_num {
			if i >> j & 1 == 0 {
				model.push(VarValue::VTrue);
			}else {
				model.push(VarValue::VFalse);
			}
		}
		if verify(clauses, &model) {
			return false;
		}
	}
	true
}

fn verify(clauses: &[Clause], model: &[VarValue]) -> bool {
	for clause in clauses {
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

	let clause_n = (rng.next_f32() * clause_mn + 1. + clause_mn * 3.).floor() as usize / 4;
	let mut total_size = 0;
	for _ in 0..clause_n {
		let mut clause = Clause::new();
		let clause_s = (rng.next_f32() * (clause_ms - 1.) + 2.).floor() as usize;
		total_size += clause_s;
		for _ in 0..clause_s {
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