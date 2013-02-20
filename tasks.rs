use task::spawn;

fn main() {
	for 100.times {
		do spawn {
			io::println("Hello");
		}
	}
}
