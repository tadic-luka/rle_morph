PROG = example
OBJ =$(PROG).o
CXXFLAGS= -O2
CXXFLAGS+=`pkg-config --cflags opencv`
LDFLAGS+=`pkg-config --libs opencv`

$(PROG): $(OBJ)
	$(CXX) -o $(PROG) $(OBJ) $(LDFLAGS)


$(OBJ): $(PROG).cpp
	$(CXX) -c $(CXXFLAGS) $<


all: $(PROG)

clean:
		- rm -rf $(PROG) *.o
