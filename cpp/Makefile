CXX = g++
CFLAGS = -O3

all: src/main.cpp
	$(CXX) $(CFLAGS) src/main.cpp -o build/ppm

view: src/main.cpp
	$(CXX) $(CFLAGS) src/main.cpp -o build/ppm && time ./build/ppm > final.ppm && \
		feh --auto-zoom -z --scale-down -g 1024x576 final.ppm 2>/dev/null

clean: build/ppm
	rm build/ppm
