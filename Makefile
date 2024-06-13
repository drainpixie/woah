CXX				= clang++

CXXFLAGS ?= -fsanitize=address -Wall -Wextra -g
CXXFLAGS += -std=c++20

CXXFLAGS += `pkg-config --cflags fmt cxxopts`
LDFLAGS  += `pkg-config --libs fmt cxxopts`

TARGET	 ?= woah
SRC				= main.cpp

all: $(TARGET)
clean:
	rm -f $(TARGET)
	rm -rf result/

$(TARGET): $(SRC)
	$(CXX) -o $@ $^ $(CXXFLAGS) $(LDFLAGS)

.PHONY: all clean
