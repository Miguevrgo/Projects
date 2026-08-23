#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "raylib.h"

#define WIDTH 1920
#define HEIGHT 1080
#define BRICKS_PER_ROW 20
#define ROWS 10

typedef struct Player {
    Vector2 position;
    Vector2 size;
    uint8_t lifes;
} Player;

typedef struct Ball {
    Vector2 pos;
    Vector2 speed;
    uint8_t radius;
} Ball;

typedef struct Brick {
    Vector2 pos;
    Vector2 size;
    bool active;
} Brick;

typedef struct GameData {
    Vector2 brick_size;
    Brick* bricks;
    Player player;
    Ball ball;
} GameData;

void InitGame(GameData* const data) {
    data->brick_size.x = ((float)WIDTH / BRICKS_PER_ROW);
    data->brick_size.y = ((float)(HEIGHT >> 1) / ROWS);

    data->bricks = malloc(sizeof(Brick) * BRICKS_PER_ROW * ROWS);
    for (size_t row = 0; row < ROWS; ++row) {
        for (size_t col = 0; col < BRICKS_PER_ROW; ++col) {
            data->bricks[(row * BRICKS_PER_ROW) + col] = (Brick){
                // We draw 0.95 * brick_width so we have to pad (1 - 0.95) / 2 -> so divide by 40
                .pos = {((float)col * data->brick_size.x) + (data->brick_size.x / 40),
                        ((float)row * data->brick_size.y) + (data->brick_size.y / 4)},
                .active = true,
            };
        }
    }

    data->player.lifes = 10;
    data->player.size.x = WIDTH * 0.1;
    data->player.size.y = HEIGHT * 0.01;
    data->player.position.x = ((float)WIDTH / 2) - (data->player.size.x / 2);
    data->player.position.y = HEIGHT - (HEIGHT * 0.1);

    data->ball.pos.x = (float)WIDTH / 2;
    data->ball.pos.y = data->player.position.y - ((float)HEIGHT / 4);
    data->ball.radius = 25;
    data->ball.speed.x = 0;
    data->ball.speed.y = 0.5f;
}

void CloseGame(GameData* const data) { free(data->bricks); }

void DrawGame(const GameData* const data) {
    BeginDrawing();

    ClearBackground(RAYWHITE);

    for (size_t row = 0; row < ROWS; ++row) {
        for (size_t col = 0; col < BRICKS_PER_ROW; ++col) {
            if (data->bricks[(row * BRICKS_PER_ROW) + col].active) {  // HACK: Maybe move inactive to end?
                DrawRectangle((int)data->bricks[(row * BRICKS_PER_ROW) + col].pos.x,
                              (int)data->bricks[(row * BRICKS_PER_ROW) + col].pos.y, (int)data->brick_size.x * 95 / 100,
                              (int)data->brick_size.y * 95 / 100, BLUE);
            }
        }
    }

    DrawRectangle((int)data->player.position.x, (int)data->player.position.y, (int)data->player.size.x,
                  (int)data->player.size.y, BLACK);

    DrawCircle((int)data->ball.pos.x, (int)data->ball.pos.y, data->ball.radius, RED);

    EndDrawing();
}

void UpdateGame(GameData* const data) {
    if (IsKeyDown(KEY_LEFT) || IsKeyDown(KEY_H) || IsKeyDown(KEY_A)) {
        data->player.position.x -= data->player.position.x >= 5 ? 5 : 0;
    }

    if (IsKeyDown(KEY_RIGHT) || IsKeyDown(KEY_L) || IsKeyDown(KEY_D)) {
        data->player.position.x += data->player.position.x + data->player.size.x <= WIDTH - 5 ? 5 : 0;
    }

    data->ball.pos.x += data->ball.speed.x;
    data->ball.pos.y += data->ball.speed.y;
}

int main(void) {
    InitWindow(WIDTH, HEIGHT, "Arkanoid");
    SetTargetFPS(165);
    GameData data = {};

    InitGame(&data);

    while (!WindowShouldClose()) {
        UpdateGame(&data);
        DrawGame(&data);
    }

    CloseGame(&data);
    CloseWindow();
}
