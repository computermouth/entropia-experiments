
all: libraybridge.a raylib/src/libraylib.a

raybridge.o: raybridge.c
	$(CC) -c raybridge.c -o raybridge.o

libraybridge.a: raybridge.o
	$(AR) rcs libraybridge.a raybridge.o

raylib/src/libraylib.a:
	$(MAKE) -C raylib/src PLATFORM=PLATFORM_DESKTOP
