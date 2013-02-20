extern mod std;

fn main() {
	for int::range(1,101) |num| {
		io::println(
			if is_fifteen(num) { ~"FizzBuzz" }
			else if is_three(num) { ~"Fizz" }
			else if is_five(num) { ~"Buzz" }
			else { int::str(num) }
		);
	}
}

fn is_three(i: int) -> bool{
	i % 3 == 0
}

fn is_five(i: int) -> bool {
	i % 5 == 0
}

fn is_fifteen(i: int) -> bool {
	is_three(i) && is_five(i)
}

#[test]
fn test_is_three_with_not_three() {
	if is_three(1) {
		fail ~"One is not three";
	}
}

#[test]
fn test_is_three_with_three() {
	if !is_three(3) {
		fail ~"Three should be three";
	}
}

#[test]
fn test_is_five_with_not_five() {
	if is_five(1) {
		fail ~"One is not five";
	}
}

#[test]
fn test_is_five_with_five() {
	if !is_five(5) {
		fail ~"five should be five";
	}
}

#[test]
fn test_is_fifteen_with_not_fifteen() {
	if is_fifteen(1) {
		fail ~"One is not fifteen";
	}
}

#[test]
fn test_is_fifteen_with_fifteen() {
	if !is_fifteen(15) {
		fail ~"fifteen should be fifteen";
	}
}
