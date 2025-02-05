#![allow(unused)]

fn main() {

	// Singed Integers
	let i0: i8 = 0; // 2**(8 - 1) ~ 2**(8 - 1) - 1
	//				    128           127
	let i1: i16 = 1;
	let i2: i32 = 1;
	let i3: i64 = 1;
	let i4: i128 = 1;
	let i5: isize = 1;

	// Unsigned Integers
	let u0: u8 = 1;	// 0 ~ 2**8 -1 = 255
	let u1: u16 = 1;
	let u2: u32 = 1;
	let u3: u64 = 1;
	let u4: u128 = 1;
	let u5: usize = 1;

	// Floats
	let f0: f32 = 0.01;
	let f1: f64 = 0.01;

	// Boolean
	let b: bool = true;

	// Characters
	let c: char = 'c';
	let e: char = '🥶';

	// Type Conversion
	let i: i32 = 1;
	let u: u32 = i as u32;
	let x: u32 = u + (i as u32);

	// Min and Max
	let min_i: i32 = i32::MIN;
	let max_i: i32 = i32::MAX;

	println!("i32 min: {min_i}");
	println!("i32 max: {max_i}");

	let min_char: char = char::MIN;
	let max_char: char = char::MAX;

	println!("i32 char: {min_char}");
	println!("i32 char: {max_char}");

	// Overflow
	let mut u: u32 = u32::MAX;
	u += 1;
	println!("overflow u32: {u}");

	// checked_add - Some(x) | None
	let u = u32::checked_add(u32::MAX, 1);
	println!("checked_add: {:?}", u);

	// wrapping_add
	let u = u32::wrapping_add(u32::MAX, 1);
	println!("wrapping_add: {:?}", u);
}
