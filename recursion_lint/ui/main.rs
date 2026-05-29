fn factorial(n: u32) -> u32 {
	if n <= 1 {
		1
	} else {
		n * factorial(n - 1)
	}
}

fn main() {
	let _ = factorial(5);
}
