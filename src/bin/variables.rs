#![allow(unused)]

fn main() {
	// immutable variable, all variables are muted by default
	let x: i32 = -123;

	// mutable variables need to be declared
	let mut y: i32 = 123;
	y += 1;

	// rust can infer the type by looking at the value
	let z = -123;
	// let w: () = 123; how to debug and find the type, terminal: cargo build to compile

	const NUM: u32 = 1;

	let x: i32 = -1; // shadowing
	let x: bool = true; // a variables type can change

	let v: Vec<_> = vec![1, 2, 3];
}
