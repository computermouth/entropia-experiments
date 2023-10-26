
all:
	make -C raylib/src PLATFORM=PLATFORM_DESKTOP
	gcc -I./libdsa main.c libdsa/vector.c -L./raylib/src -lraylib -lm
