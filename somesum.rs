fn main() {
	let mut s = 0;
	let n = 1111;
	let k = 2;
	for i in 1..(n+1) {
		s += u32::pow(i,k);
		// println!("i = {}, s = {}", i, s);
	}
	println!("sum = {}", s);

	let somesum = n*(n+1)*(2*n+1)/6;
	println!("somesum = {}", somesum);
}

//  Rust  rustc somesum.rs && ./somesum 
// sum = 457727556
// somesum = 457727556
