LIBS = -L . -lftd3xx
# Static link to D3XX library, please use Ubuntu 14.04 to compile
#LIBS = -L . -lftd3xx-static -lstdc++-static

ifneq ("$(wildcard ftd3xx.dll)","")
# === Windows ===
ifneq (,$(findstring PE32+,$(shell file ftd3xx.dll)))
	CROSS_COMPILE := x86_64-w64-mingw32-
else
	CROSS_COMPILE := i686-w64-mingw32-
endif
AS = $(CROSS_COMPILE)as
LD = $(CROSS_COMPILE)ld
CC = $(CROSS_COMPILE)gcc-posix
CXX = $(CROSS_COMPILE)g++-posix
OBJDUMP = $(CROSS_COMPILE)objdump

DEMO0=streamer.exe
DEMO1=rw.exe
DEMO2=file_transfer.exe
DEMO3=asyncstreamer.exe
DEMO4=async-loopback.exe
DEMO5=FTD3xx_Notification.exe
CXXLIBS = -static -lstdc++
# === End of Windows ===
else
# === Linux & macOS ===
DEMO0=streamer
DEMO1=rw
DEMO2=file_transfer
DEMO3=asyncstreamer
DEMO4=async-loopback
DEMO5=FTD3xx_Notification
ifneq ("$(wildcard libftd3xx.dylib)","")
# == macOS ==
ARCH=-m64
CXXLIBS =-lc++
# == End of macOS ==
else
# == Linux ==
ifneq (,$(findstring 64-bit,$(shell file libftd3xx.so)))
ARCH=-m64
else
ARCH=-m32
endif
LIBS += -pthread -lrt
CXXLIBS = -lstdc++
# == End of Linux ==
endif
# === End of Linux & macOS ===
endif

COMMON_FLAGS = -ffunction-sections -fmerge-all-constants $(ARCH)
COMMON_CFLAGS = -g -O3 -Wall -Wextra $(COMMON_FLAGS)
CFLAGS = -std=c99  $(COMMON_CFLAGS) -D_POSIX_C_SOURCE
CXXFLAGS = -std=c++11 $(COMMON_CFLAGS)

all: $(DEMO0) $(DEMO1) $(DEMO2) $(DEMO3) $(DEMO4) $(DEMO5)
	
$(DEMO0): streamer.o
	$(CC) $(COMMON_FLAGS) -o $@ $^ $(CXXLIBS) $(LIBS)

$(DEMO1): rw.o
	$(CC) $(COMMON_FLAGS) -o $@ $^ $(LIBS)

$(DEMO2): file_transfer.o
	$(CC) $(COMMON_FLAGS) -o $@ $^ $(CXXLIBS) $(LIBS)

$(DEMO3): asyncstreamer.o
	$(CC) $(COMMON_FLAGS) -o $@ $^ $(CXXLIBS) $(LIBS)

$(DEMO4): async-loopback.o
	$(CC) $(COMMON_FLAGS) -o $@ $^ $(CXXLIBS) $(LIBS)

$(DEMO5): FTD3xx_Notification.o
	$(CC) $(COMMON_FLAGS) -o $@ $^ $(CXXLIBS) $(LIBS)

clean:
	-rm -f streamer.o rw.o file_transfer.o asyncstreamer.o async-loopback.o FTD3xx_Notification.o $(DEMO0) $(DEMO1) $(DEMO2) $(DEMO3) $(DEMO4) $(DEMO5)
