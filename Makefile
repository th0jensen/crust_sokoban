.PHONY: all linux macos clean run

OUT = sokoban
SRC = src/main.rs
RUSTC_FLAGS = --edition 2021 -g -C opt-level=z -C panic="abort"

UNAME_S := $(shell uname -s)

MACOS_ARGS = -L/opt/homebrew/lib -lraylib -lc -lm -framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL
LINUX_RAYLIB_LIBS := $(shell pkg-config --libs raylib 2>/dev/null)
ifeq ($(strip $(LINUX_RAYLIB_LIBS)),)
LINUX_RAYLIB_LIBS = -L/usr/local/lib -lraylib -lGL -lm -lpthread -ldl -lrt -lX11
endif
LINUX_ARGS = $(LINUX_RAYLIB_LIBS) -lc

ifeq ($(UNAME_S),Linux)
LINK_ARGS = $(LINUX_ARGS)
else ifeq ($(UNAME_S),Darwin)
LINK_ARGS = $(MACOS_ARGS)
else
$(error Unsupported OS: $(UNAME_S))
endif

all: $(OUT)

$(OUT): $(SRC)
	rustc $(RUSTC_FLAGS) -C link-args="$(LINK_ARGS)" $(SRC) -o $(OUT)

linux macos: all

run: all
	./$(OUT)

clean:
	rm -f $(OUT)
	rm -f *.o
