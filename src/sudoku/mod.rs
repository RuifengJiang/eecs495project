use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::fmt;

use sat::sat_lib::*;
use sudoku::mapper::*;

mod mapper;

pub fn sudoku() {
	let mut s = Mapper::new();
	s.build_clauses();
	File::create("foo.txt").unwrap();
	let mut file = OpenOptions::new().read(true).write(true).append(true).open("./foo.txt").unwrap();
	file.write(s.out.as_bytes()).unwrap();

	let file_puzzle = File::open("./SudokuPuzzle.txt").unwrap();
	let reader = BufReader::new(file_puzzle);

	let mut input = Vec::new();

	for line in reader.lines() {
    	match line {
    		Ok(c) => {
    			if c.starts_with("c")  || c.len() == 0 { continue }
    			let mut iter = c.split_whitespace();
    			let mut temp = format!("{}{}{} ", iter.next().unwrap(),iter.next().unwrap(),iter.next().unwrap());
    			// println!("{:?}",temp );
    			file.write(temp.as_bytes()).unwrap();
    			temp.pop();
    			input.push(temp.parse::<usize>().unwrap());
    			file.write("0\n".as_bytes()).unwrap(); 
    		},
    		Err(_) => {},
    	}
	}

	let mut v_ori = Vec::new();
	for _ in 0..9{
		let mut v_temp = Vec::new();
		for _ in 0..9{
			v_temp.push(" ".to_string());
		}
		v_ori.push(v_temp);
	}

	for i in input.iter(){
		v_ori[i/100 -1][(i%100)/10 -1] = format!("{}",i%10);
	}
 
	print_sudoku(&v_ori);


	let mut solver = Solver::new();
	let file_new = File::open("./foo.txt").unwrap();
	let reader = BufReader::new(file_new);

	for line in reader.lines() {
		match line{
			Ok(c) => {
				let mut iter = c.split_whitespace();
				let mut lits = Vec::<Lit>::new();
				while let Some(v) = iter.next(){
					if let Ok(num) = v.parse::<i32>() {
						if num < 0{
							lits.push(!Lit::new(Var::new((-num) as usize)));
						}else if num >0 {
							lits.push(Lit::new(Var::new(num as usize)));
						}
					}
				} 
				if lits.len() != 0 {
					solver.add_clause_from_lits(lits).unwrap();
				}

			},
			Err(_) => {},
		}
	}
	solver.solve();
	let res = solver.get_model();

	let mut v = Vec::new();
	for i in 0..1000{
		if res[i] == VarValue::VTrue{
			v.push(i)
		}
	}

	let mut v2 = Vec::new();
	for i in 0..9{
		let mut v_temp = Vec::new();
		for j in 0..9{
			v_temp.push(v[i*9+j]%10);
		}
		v2.push(v_temp);
	}
	print_sudoku(&v2);
}

fn print_sudoku<T>(v: &Vec<Vec<T>>) 
where T: fmt::Display {
	// println!("{:?}",input );
	for i in 0..9{
		println!("-------------------------------------");
		for j in 0..9{
			print!("| {} ", v[i][j]);
		}
		print!("|\n");
	}
	println!("-------------------------------------");
}

