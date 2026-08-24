#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>

#include "raylib.h"

#define WIDTH 1920
#define HEIGHT 1080
#define BRICKS_PER_ROW 20
#define PLAYER_SPEED 825
#define ROWS 10
#define MAX_SPEED 1
#define THETA_MAX (PI / 3)

const Color COLORS[6] = {
    {0xf3, 0x8b, 0xa8, 0xff},  //
    {0xfa, 0xb3, 0x87, 0xff},  //
    {0xf9, 0xe2, 0xaf, 0xff},  //
    {0xa6, 0xe3, 0xa1, 0xff},  //
    {0x89, 0xb4, 0xfa, 0xff},  //
    {0xcb, 0xa6, 0xf7, 0xff}   //
};

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
    bool game_over;
} GameData;

void RestartBall(GameData* const data) {
    data->ball.pos.x = (float)WIDTH / 2;
    data->ball.pos.y = data->player.pos.y - ((float)HEIGHT / 4);
    data->ball.radius = 25;

    const float u = (((float)rand() * 2.0F / RAND_MAX) - 1.0F);
    data->ball.speed.x = cbrtf(u) * 825.0F;
    data->ball.speed.y = 100.0F;
}

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
    data->player.pos.y = HEIGHT - (HEIGHT * 0.05);

    RestartBall(data);

    data->game_over = false;
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
                              (int)data->brick_size.y * 95 / 100, COLORS[row % 6]);
            }
        }
    }

    DrawRectangle((int)data->player.pos.x, (int)data->player.pos.y, (int)data->player.size.x, (int)data->player.size.y,
                  BLACK);

    DrawCircle((int)data->ball.pos.x, (int)data->ball.pos.y, data->ball.radius, RED);

    DrawText(TextFormat("Lifes: %i", data->player.lifes), 10, HEIGHT - 30, 30, BLACK);
    if (data->game_over) {
        DrawText("GAME OVER", (WIDTH / 2) - (MeasureText("GAME OVER", 60) / 2), HEIGHT / 2, 60, MAROON);
    }

    EndDrawing();
}

void ResolveCollisions(GameData* const data) {
    if (CheckCollisionCircleRec(data->ball.pos, data->ball.radius,
                                (Rectangle){
                                    .x = data->player.pos.x,
                                    .y = data->player.pos.y,
                                    .width = data->player.size.x,
                                    .height = data->player.size.y,
                                })) {
        const float T =
            (data->ball.pos.x - (data->player.pos.x + (data->player.size.x / 2))) / (data->player.size.x / 2);
        const float THETA = T * THETA_MAX;
        const float MODULE =
            sqrtf((data->ball.speed.x * data->ball.speed.x) + (data->ball.speed.y * data->ball.speed.y));
        data->ball.speed.x = MODULE * sinf(THETA);
        data->ball.speed.y = -MODULE * cosf(THETA);
    }

    // HACK: This could be optimized (if brick->inactive's) are sent to the end
    // or even checking collisions only for the bricks which are close to the ball
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

bool BricksLeft(const GameData* const data) {
    for (size_t i = 0; i < (size_t)ROWS * BRICKS_PER_ROW; ++i) {
        if (data->bricks[i].active) {
            return true;
        }
    }

    return false;
}

void UpdateGame(GameData* const data) {
    if (data->game_over) {
        return;
    }

    const float DT = GetFrameTime();

    if (IsKeyDown(KEY_LEFT) || IsKeyDown(KEY_H) || IsKeyDown(KEY_A)) {
        data->player.pos.x -= PLAYER_SPEED * DT;
        data->player.pos.x = (data->player.pos.x < 0) ? 0 : data->player.pos.x;
    }

    if (IsKeyDown(KEY_RIGHT) || IsKeyDown(KEY_L) || IsKeyDown(KEY_D)) {
        data->player.pos.x += PLAYER_SPEED * DT;
        data->player.pos.x =
            (data->player.pos.x > WIDTH - data->player.size.x) ? WIDTH - data->player.size.x : data->player.pos.x;
    }

    data->ball.pos.x += data->ball.speed.x * DT;
    data->ball.pos.y += data->ball.speed.y * DT;

    if ((int)data->ball.pos.x + data->ball.radius >= WIDTH || (int)data->ball.pos.x - data->ball.radius <= 0) {
        data->ball.speed.x *= -1;
    }
    if ((int)data->ball.pos.y - data->ball.radius <= 0) {
        data->ball.speed.y *= -1;
    }

    if ((int)data->ball.pos.y + data->ball.radius >= HEIGHT) {
        RestartBall(data);
        data->player.lifes -= 1;
    }

    if (data->player.lifes == 0 || !BricksLeft(data)) {
        data->game_over = true;
    }

    ResolveCollisions(data);
}

int main(void) {
    InitWindow(WIDTH, HEIGHT, "Arkanoid");
    SetTargetFPS(165);
    srand(time(nullptr));
    GameData data = {};

    InitGame(&data);

    while (!WindowShouldClose()) {
        UpdateGame(&data);
        DrawGame(&data);
    }

    CloseGame(&data);
    CloseWindow();
}
