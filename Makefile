all: test run

run: fizzbuzz
	./fizzbuzz

fizzbuzz: fizzbuzz.rs
	rustc fizzbuzz.rs

test: test-fizzbuzz
	./test-fizzbuzz

test-fizzbuzz: fizzbuzz.rs
	rustc --test fizzbuzz.rs -o test-fizzbuzz

tasks: tasks.rs
	rustc tasks.rs

run-tasks: tasks
	./tasks

pipes: pipes.rs
	rustc pipes.rs

run-pipes: pipes
	./pipes

duplex: duplex.rs
	rustc duplex.rs

run-duplex: duplex
	./duplex
