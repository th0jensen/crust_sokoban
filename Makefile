.PHONY: clean

ARGS = "-L/opt/homebrew/lib -lraylib -lc -lm -framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL"
OUT = sokoban

$(OUT): src/main.rs
	rustc --edition 2021 -g -C opt-level=z -C link-args=$(ARGS) -C panic="abort" src/main.rs -o $(OUT)

clean:
	rm -f sokoban
	rm -f *.o
