fn main() {
	let mut i = 0;

	'epic: loop {
		i += 1;
		if i > 1000 {
			continue 'epic;
		}
		break;
	}
}
