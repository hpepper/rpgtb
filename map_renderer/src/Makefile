all: ut build

RUN_NAME=maprenderer

CXX=g++
CFLAGS=-Wall -export-dynamic -g -O0
CXXFLAGS=-std=c++20
LIBS=-ltinyxml2 -ldl
# -ldl to get dladdr()

# See: https://stackoverflow.com/questions/7004702/how-can-i-create-a-makefile-for-c-projects-with-src-obj-and-bin-subdirectories
SOURCES  := trace.cpp \
			xml_if.cpp \
			map_manager.cpp \
			svg_renderer.cpp

INCLUDES := $(SOURCES:.cpp=.h)
OBJECTS  := $(SOURCES:.cpp=.o)

build: $(RUN_NAME)

$(RUN_NAME): $(RUN_NAME).o $(OBJECTS)
	$(CXX) $(CFLAGS) $(CXXFLAGS) -o $@ $(RUN_NAME).o $(OBJECTS) $(LIBS)

ut:
	cd unittests && make all

clean:
	- rm $(RUN_NAME)
	- rm *.o
	- cd unittests && make clean

%.o: %.cpp %.h
	$(CXX) -c $(CFLAGS) $(CXXFLAGS) $< -o $@