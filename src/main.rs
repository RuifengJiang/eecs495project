use std::env::args;

mod test;
mod sudoku;

extern crate sat;
extern crate rand;
extern crate time;

fn main() {
	let mut args = args();
	args.next();
	
	if let Some(arg) = args.next() {
		if arg == "test" {
			test::test();
		}else if arg == "sudoku" {
			sudoku::sudoku(args.next());
		}
	}
}