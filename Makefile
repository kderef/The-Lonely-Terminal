CC = gcc
CFLAGS = -Iraylib/src -I. -Wall -Wextra -s -O2

LIBRAYLIB = raylib/src/libraylib.a

BIN = bin/game

O_GAME = bin/game.o
O_SKBX = bin/skybox.o
O_PLCM = bin/player_camera.o
O_MENU = bin/menu.o
O_VDEO = bin/video.o
O_AUDI = bin/audio.o
O_ASST = bin/asset.o
O_PLYR = bin/player.o

ifdef OS
	# Windows
	OSFLAGS = -lgdi32 -lwinmm
else
	ifeq ($(shell uname), Linux)
		# TODO
	endif
	ifeq ($(shell uname), Darwin)
		OSFLAGS = -framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL
	endif
endif

FLAGS = $(CFLAGS) $(OSFLAGS)

$(BIN): $(LIBRAYLIB) $(O_GAME) $(O_SKBX) $(O_PLCM) $(O_MENU) $(O_VDEO) $(O_AUDI) $(O_ASST) $(O_PLYR) src/main.c 
	$(CC) src/main.c $(O_GAME) $(O_SKBX) $(O_PLCM) $(O_MENU) $(O_VDEO) $(O_AUDI) $(O_ASST) $(O_PLYR) $(LIBRAYLIB) -o $(BIN) $(FLAGS)

$(O_SKBX): src/skybox.*
	$(CC) $(CFLAGS) -c src/skybox.c -o $(O_SKBX)

$(O_GAME): src/game.*
	$(CC) $(CFLAGS) -c src/game.c -o $(O_GAME)

$(O_PLCM): src/player_camera.*
	$(CC) $(CFLAGS) -c src/player_camera.c -o $(O_PLCM)

$(O_MENU): src/menu.*
	$(CC) $(CFLAGS) -c src/menu.c -o $(O_MENU)

$(O_VDEO): src/video.*
	$(CC) $(CFLAGS) -c src/video.c -o $(O_VDEO)

$(O_AUDI): src/audio.*
	$(CC) $(CFLAGS) -c src/audio.c -o $(O_AUDI)

$(O_ASST): src/asset.*
	$(CC) $(CFLAGS) -c src/asset.c -o $(O_ASST)

$(O_PLYR): src/player.*
	$(CC) $(CFLAGS) -c src/player.c -o $(O_PLYR)
	
$(LIBRAYLIB): raylib/src/*.[ch]
	make -C raylib/src
