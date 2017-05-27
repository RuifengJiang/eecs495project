 #![warn(unused_variables)]

static NUM_VARS: u32 = 9;
static SQUARE_SIZE: u32 = 9;
static BOX_SIZE: u32 = 3;

#[derive(Debug)]
pub struct Mapper {
	pub out: String,
}

impl Mapper {
		pub fn new() -> Self {
			Mapper{out: String::new()}

		}

		pub fn build_clauses(&mut self){
			self.constraint_1a();
			self.constraint_1b();
			self.constraint_2a();
			self.constraint_2b();
			self.constraint_3a();
			self.constraint_3b();
			self.constraint_4();
			self.constraint_5();
		}

		pub fn constraint_1a(&mut self){
			for k in 1..NUM_VARS+1{
				for i in 1..(SQUARE_SIZE+1){
					for j in 1..(SQUARE_SIZE+1){
						self.out.push_str(&format!("{}{}{} ", i,j,k));
					} 
					self.out.push_str("0\n"); 
				}
			}
		}

		pub fn constraint_1b(&mut self){
			for k in 1..NUM_VARS+1{
				for i in 1..(SQUARE_SIZE+1){
					for j in 1..(SQUARE_SIZE+1){
						self.out.push_str(&format!("{}{}{} ", j,i,k));
					} 
					self.out.push_str("0\n"); 
				}
			}
		}


		pub fn constraint_2a(&mut self){
			for k in 1..(NUM_VARS+1){
				for n in 1..(SQUARE_SIZE+1){
					for j in 1..(SQUARE_SIZE+1){
						// let mut ptr: u32 = 1;
						for i in (j+1)..(SQUARE_SIZE+1){
							self.out.push_str(&format!("-{}{}{} ", n, j, k));
							self.out.push_str(&format!("-{}{}{} ", n, i, k));
							self.out.push_str("0\n");
							// ptr = ptr+1;
						} 
					}
				}
			}
		}


		pub fn constraint_2b(&mut self){
			for k in 1..(NUM_VARS+1){
				for n in 1..(SQUARE_SIZE+1){
					for j in 1..(SQUARE_SIZE+1){
						// let mut ptr: u32 = 1;
						for i in (j+1)..(SQUARE_SIZE+1){
							self.out.push_str(&format!("-{}{}{} ", j, n, k));
							self.out.push_str(&format!("-{}{}{} ", i, n, k));
							self.out.push_str("0\n");
							// ptr = ptr + 1;
						} 
					}
				}
			}
		}


		pub fn constraint_3a(&mut self){
			for k in 1..(NUM_VARS+1){
				for m in 1..(SQUARE_SIZE+1){
					if m % BOX_SIZE != 1 { continue; }
					for n in 1..(SQUARE_SIZE+1){
						if n % BOX_SIZE != 1 { continue; }
						for i in 0..BOX_SIZE {
							for j  in 0..BOX_SIZE {
								self.out.push_str(&format!("-{}{}{} ", m+i, n+j, k));
							}
						}
						self.out.push_str("0\n");
					}
				}
			}
		}


		pub fn constraint_3b(&mut self){
			for k in 1..(NUM_VARS+1){
				for m in 1..(SQUARE_SIZE+1){
					if m % BOX_SIZE != 1 { continue; }
					for n in 1..(SQUARE_SIZE+1){
						if n % BOX_SIZE != 1 { continue; }

						let mut v = Vec::new();

						for i in 0..BOX_SIZE {
							for j  in 0..BOX_SIZE {
								v.push(format!("{}{}{} ", m+i, n+j, k));
							}
						}

						for j in 0..v.len(){
							// let mut ptr: usize = 1;
							for i in (j+1)..v.len(){
								self.out.push_str(&format!("-{} ",v[j]));
								self.out.push_str(&format!("-{} ",v[i]));
								self.out.push_str("0\n");
								// ptr =ptr +1;
							}
						}
					}
				}
			}
		}


		pub fn constraint_4(&mut self){
			for i in 1..(SQUARE_SIZE+1){
				for j in 1..(SQUARE_SIZE+1){
					for k in 1..(SQUARE_SIZE+1){
						self.out.push_str(&format!("{}{}{} ", i, j, k));
					}
					self.out.push_str("0\n");
				}
			}
		}

		pub fn constraint_5(&mut self){
			for i in 1..(SQUARE_SIZE+1){
				for j in 1..(SQUARE_SIZE+1){
					for k in 1..(NUM_VARS+1){
						for m in (k+1)..(SQUARE_SIZE+1){
						self.out.push_str(&format!("-{}{}{} ", i, j, k));
						self.out.push_str(&format!("-{}{}{} ", i, j, m));
						self.out.push_str("0\n");
						}
					}
				}
			}
		}

}


#[cfg(test)]
mod tests {
		#[test]
		fn it_works() {
			let mut out = "test:".to_string();
			out.push_str(&format!("{}{}{}", 1,2,3));
			assert_eq!("test:123 ",out);
		}
}








