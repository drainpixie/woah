CXX				= clang++

CXXFLAGS ?= -fsanitize=address -Wall -Wextra -g
CXXFLAGS += -std=c++20

# CXXFLAGS += `pkg-config --cflags `
# LDFLAGS  += `pkg-config --libs `

TARGET	 ?= woah
SRC				= main.cpp

all: $(TARGET)
clean:
	rm -f $(TARGET)
	rm -rf result/

$(TARGET): $(SRC)
	$(CXX) -o $@ $^ $(CXXFLAGS) $(LDFLAGS)

.PHONY: all clean
