CFLAGS = -std=c++17 -O2 #-DNDEBUG
LDFLAGS = -lglfw -lvulkan -ldl -lpthread -lX11 -lXxf86vm -lXrandr -lXi


VulkanTest: triangle.cpp
	g++ $(CFLAGS) $(DEFINES) -o VulkanTest triangle.cpp $(LDFLAGS)

.PHONY: test clean

test: VulkanTest
	./VulkanTest

clean:
	rm -f VulkanTest
