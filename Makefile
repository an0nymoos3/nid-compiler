TARGET_DIR = bin/

# Builds in release mode
build:
	@if [ ! -d $(TARGET_DIR) ]; then \
		mkdir $(TARGET_DIR); \
	fi
	echo "Building compiler..."
	cargo build --release --quiet
	cp target/release/compiler bin/
	echo "Building assembler..."
	g++ -O3 src/assembler/assembler_main.cpp -o bin/assmebler

# Remove all build content
clean:
	rm -rf target/
	rm -rf bin/
