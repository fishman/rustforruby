extern mod std;
use task::spawn;
use std::comm::DuplexStream;

fn plus_one(channel: &DuplexStream<int, int>) {
	let mut value: int;
	loop {
		value = channel.recv();
		channel.send(value + 1);
		if value == 0 { break; }
	}
}

fn main() {
	let (from_child, to_child) = DuplexStream();

	do spawn |move to_child| {
		plus_one(&to_child);
	};

	from_child.send(22);
	from_child.send(23);
	from_child.send(24);
	from_child.send(25);
	from_child.send(0);

	for 4.times {
		let answer = from_child.recv();
		io::println(int::str(answer));
	}
}
