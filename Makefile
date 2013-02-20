all: test run

run: fizzbuzz
	./fizzbuzz

fizzbuzz: fizzbuzz.rs
	rustc fizzbuzz.rs

test: test-fizzbuzz
	./test-fizzbuzz

test-fizzbuzz: fizzbuzz.rs
	rustc --test fizzbuzz.rs -o test-fizzbuzz
