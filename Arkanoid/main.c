#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "raylib.h"

#define WIDTH 1920
#define HEIGHT 1080
#define BRICKS_PER_ROW 20
#define ROWS 10
#define MAX_SPEED 1

typedef struct Player {
    Vector2 pos;
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
    data->player.pos.x = ((float)WIDTH / 2) - (data->player.size.x / 2);
    data->player.pos.y = HEIGHT - (HEIGHT * 0.1);

    data->ball.pos.x = (float)WIDTH / 2;
    data->ball.pos.y = data->player.pos.y - ((float)HEIGHT / 4);
    data->ball.radius = 25;
    data->ball.speed.x = -2;
    data->ball.speed.y = 0.3F;
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

    DrawRectangle((int)data->player.pos.x, (int)data->player.pos.y, (int)data->player.size.x, (int)data->player.size.y,
                  BLACK);

    DrawCircle((int)data->ball.pos.x, (int)data->ball.pos.y, data->ball.radius, RED);

    EndDrawing();
}

void ResolveCollisions(GameData* const data) {
    // FIX: Too much speed wrong angles
    if (CheckCollisionCircleRec(data->ball.pos, data->ball.radius,
                                (Rectangle){
                                    .x = data->player.pos.x,
                                    .y = data->player.pos.y,
                                    .width = data->player.size.x,
                                    .height = data->player.size.y,
                                })) {
        const float FACTOR = fabsf(data->ball.speed.x) <= MAX_SPEED || fabsf(data->ball.speed.y) <= MAX_SPEED ? -3 : -1;
        data->ball.speed.y *= FACTOR;
        data->ball.speed.x = (data->ball.pos.x - data->player.pos.x) / (data->player.size.x / 2) * FACTOR;
    }

    for (size_t row = 0; row < ROWS; ++row) {
        for (size_t col = 0; col < BRICKS_PER_ROW; ++col) {
            Brick* brick = &data->bricks[(row * BRICKS_PER_ROW) + col];
            if (brick->active) {
                Rectangle rect = {.x = brick->pos.x,
                                  .y = brick->pos.y,
                                  .width = data->brick_size.x * 0.95F,
                                  .height = data->brick_size.y * 0.95F};

                if (CheckCollisionCircleRec(data->ball.pos, data->ball.radius, rect)) {
                    brick->active = false;
                    data->ball.speed.y *= -1;
                    return;
                }
            }
        }
    }
}

void UpdateGame(GameData* const data) {
    if (IsKeyDown(KEY_LEFT) || IsKeyDown(KEY_H) || IsKeyDown(KEY_A)) {
        data->player.pos.x -= data->player.pos.x >= 5 ? 5 : 0;
    }

    if (IsKeyDown(KEY_RIGHT) || IsKeyDown(KEY_L) || IsKeyDown(KEY_D)) {
        data->player.pos.x += data->player.pos.x + data->player.size.x <= WIDTH - 5 ? 5 : 0;
    }

    data->ball.pos.x += data->ball.speed.x;
    data->ball.pos.y += data->ball.speed.y;

    if ((int)data->ball.pos.x + data->ball.radius >= WIDTH || (int)data->ball.pos.x - data->ball.radius <= 0) {
        data->ball.speed.x *= -1;
    }
    if ((int)data->ball.pos.y - data->ball.radius <= 0) {
        data->ball.speed.y *= -1;
    }

    if ((int)data->ball.pos.y + data->ball.radius >= HEIGHT) {
        data->ball.speed.x *= 0.5F;
        data->ball.speed.y *= 0.5F;
        data->ball.pos.x = (float)WIDTH / 2;
        data->ball.pos.y = data->player.pos.y - ((float)HEIGHT / 4);
        data->player.lifes -= 1;
    }

    ResolveCollisions(data);
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
