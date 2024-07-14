# Compiler
CXX := clang++

# Compiler flags
CXXFLAGS := -L ../target/debug -lcutelights -I ../target/include

# Source files
SRCS := main.cc

# Object files
OBJS := $(SRCS:.cc=.o)

# Executable name
TARGET := main

# Default target
all: $(TARGET)

cargo:
	cargo build

# Rule to build the executable
$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) $^ -o $@

# Rule to build object files
%.o: %.cc
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean rule
clean:
	rm -f $(OBJS) $(TARGET)

run: $(TARGET)
	make cargo
	LD_LIBRARY_PATH=../target/debug fish -c 'time ./$(TARGET)'

run-no-copile:
	LD_LIBRARY_PATH=../target/debug fish -c 'time ./$(TARGET)'

.PHONY: all clean run cargo run-no-copile