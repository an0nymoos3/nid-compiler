# Makes make command silent. Use "make VERBOSE=1" to print every step.
ifndef VERBOSE
.SILENT:
endif

TARGET_DIR = bin/
CPP_OBJS = $(wildcard assembler/src/*.cpp) $(wildcard assembler/src/utils/*.cpp)

# Builds in release mode
all: compiler assembler

compiler:
	@if [ ! -d $(TARGET_DIR) ]; then \
		mkdir $(TARGET_DIR); \
	fi
	echo "Building compiler..."
	cargo build --release --quiet --manifest-path compiler/Cargo.toml 
	cp target/release/compiler $(TARGET_DIR)

assembler: 
	@if [ ! -d $(TARGET_DIR) ]; then \
		mkdir $(TARGET_DIR); \
	fi
	echo "Building assembler..."
	g++ -O3 $(CPP_OBJS) -o $(TARGET_DIR)assembler

# Remove all build content
clean:
	echo "rm -rf target/ && rm -rf bin/"
	rm -rf compiler/target/
	rm -rf bin/
