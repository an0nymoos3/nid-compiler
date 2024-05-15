TARGET_DIR=bin/
CPP_OBJS=$(wildcard assembler/src/utils/*.cpp) $(wildcard assembler/src/*.cpp)

.PHONY: all compiler assembler clean

# Makes make command silent. Use "make VERBOSE=1" to print every step.
ifndef VERBOSE
.SILENT:
endif

# Builds in release mode
all:	assembler	 compiler

compiler: | $(TARGET_DIR)
	echo "Building compiler..."
	cargo build --release --quiet --manifest-path compiler/Cargo.toml 
	cp compiler/target/release/compiler $(TARGET_DIR)

assembler: | $(TARGET_DIR)
	echo "Building assembler..."
	g++ -O3 $(CPP_OBJS) -o $(TARGET_DIR)assembler

# Remove all build content
clean:
	echo "Cleaning..."
	rm -rf compiler/target/
	rm -rf bin/

$(TARGET_DIR):
	@mkdir -p $(TARGET_DIR)

