CC         ?= gcc
NAME       := arkanoid
SRCS       := $(wildcard src/*.c) main.c

CFLAGS     := -Wall -Wextra -Wpedantic -std=c23 $(shell pkg-config --cflags raylib)
LDLIBS     := -lm $(shell pkg-config --libs raylib)
BUILD_DIR  := target

REL_DIR    := $(BUILD_DIR)/release
REL_TARGET := $(REL_DIR)/$(NAME)
REL_OBJS   := $(patsubst %.c, $(REL_DIR)/%.o, $(SRCS))
REL_CFLAGS := $(CFLAGS) -O2

DBG_DIR    := $(BUILD_DIR)/debug
DBG_TARGET := $(DBG_DIR)/$(NAME)
DBG_OBJS   := $(patsubst %.c, $(DBG_DIR)/%.o, $(SRCS))
DBG_CFLAGS := $(CFLAGS) -g -O0 -DDEBUG


.PHONY: all release debug run clean

all: release
release: $(REL_TARGET)
debug: $(DBG_TARGET)

$(REL_TARGET): $(REL_OBJS)
	@mkdir -p $(@D)
	$(CC) $^ -o $@ $(LDLIBS)

$(REL_DIR)/%.o: %.c
	@mkdir -p $(@D)
	$(CC) $(REL_CFLAGS) -MMD -MP -c $< -o $@

$(DBG_TARGET): $(DBG_OBJS)
	@mkdir -p $(@D)
	$(CC) $^ -o $@ $(LDLIBS)

$(DBG_DIR)/%.o: %.c
	@mkdir -p $(@D)
	$(CC) $(DBG_CFLAGS) -MMD -MP -c $< -o $@

-include $(REL_OBJS:.o=.d) $(DBG_OBJS:.o=.d)

run: release
	@./$(REL_TARGET)

clean:
	rm -rf $(BUILD_DIR)

