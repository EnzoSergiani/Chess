CC = gcc

CFLAGS = -Wall -I./inc

SRC_DIR = src
BIN_DIR = bin
INC_DIR = inc
DATA_DIR = data

SRCS = $(wildcard $(SRC_DIR)/*.c) $(SRC_DIR)/main.c

OBJS = $(patsubst $(SRC_DIR)/%.c, $(BIN_DIR)/%.o, $(SRCS))

TARGET = $(BIN_DIR)/main

$(TARGET): $(OBJS)
	$(CC) $^ -o $@ $(CFLAGS)

$(BIN_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(shell mkdir -p $(BIN_DIR) $(DATA_DIR))

clean:
	rm -rf $(BIN_DIR)

run: $(TARGET)
	./$(TARGET)
