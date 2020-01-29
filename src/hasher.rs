use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn run(){
	let mut hasher = DefaultHasher::new();
	let numbers = [6, 28, 496, 8128];
	Hash::hash_slice(&numbers, &mut hasher);
	println!("Hash is {:x}!", hasher.finish());	
}
