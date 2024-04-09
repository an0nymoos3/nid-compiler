# Builds in release mode
build:
	echo "Building compiler..."
	cargo build --release --quiet
	cp target/release/compiler bin/
	echo "Building assembler..."
	g++ -O3 assembler/assembler_main.cpp -o bin/assmebler

# Runs a debug build
run: 
	echo "Building compiler..."
	cargo build --quiet
	cp target/release/compiler bin/
	echo "Building assembler..."
	g++ assembler/assembler_main.cpp -o bin/assmebler
	./bin/compiler

# Remove all build content
clean:
	rm -rf target/
	rm -rf bin/
