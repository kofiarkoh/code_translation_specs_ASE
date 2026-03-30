#include <stdio.h>
#include <stdlib.h>

#define MAX_HEIGHT 51
#define MAX_WIDTH 51

typedef enum {
    GAME_CONTINUE,
    GAME_CLEARED,
    GAME_DEATH_HOLE,
    GAME_DEATH_BLOCK,
    GAME_DEATH_GOAL,
    GAME_GAVE_UP
} GameStatus;

typedef struct {
    char grid[MAX_HEIGHT][MAX_WIDTH];
    int height;
    int width;
    int player_x;
    int player_y;
} Game;

void initialize_game(Game *game, int height, int width, char grid[MAX_HEIGHT][MAX_WIDTH]) {
    if (height <= 0 || width <= 0) {
        game->height = 0;
        game->width = 0;
        return; // Terminate gracefully if dimensions are zero
    }
    game->height = height;
    game->width = width;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            game->grid[i][j] = grid[i][j];
            if (grid[i][j] == 'P') { // Assuming 'P' marks the player's starting position
                game->player_x = j;
                game->player_y = i;
            }
        }
    }
}

GameStatus move_player(Game *game, int new_x, int new_y) {
    // Check bounds
    if (new_x < 0 || new_x >= game->width || new_y < 0 || new_y >= game->height) {
        return GAME_CONTINUE; // Ignore out-of-bounds movement
    }

    char target_cell = game->grid[new_y][new_x];

    // Handle game-over conditions
    if (target_cell == 'H') { // Assuming 'H' represents a hole
        return GAME_DEATH_HOLE;
    } else if (target_cell == 'B') { // Assuming 'B' represents a block
        return GAME_DEATH_BLOCK;
    } else if (target_cell == 'G') { // Assuming 'G' represents a goal
        return GAME_DEATH_GOAL;
    } else if (target_cell == ' ') { // Assuming empty space is valid
        // Move player
        game->grid[game->player_y][game->player_x] = ' '; // Clear old position
        game->player_x = new_x;
        game->player_y = new_y;
        game->grid[new_y][new_x] = 'P'; // Set new position
    }

    return GAME_CONTINUE;
}

void print_game_status(GameStatus status) {
    switch (status) {
        case GAME_CLEARED:
            printf("Game Cleared!\n");
            break;
        case GAME_DEATH_HOLE:
            printf("Death by Hole!\n");
            break;
        case GAME_DEATH_BLOCK:
            printf("Death by Block!\n");
            break;
        case GAME_DEATH_GOAL:
            printf("Death by Walking Goal!\n");
            break;
        case GAME_GAVE_UP:
            printf("Game Given Up!\n");
            break;
        default:
            break;
    }
}

int main() {
    Game game;
    char initial_grid[MAX_HEIGHT][MAX_WIDTH] = {
        {' ', ' ', ' ', ' ', ' '},
        {' ', 'P', ' ', 'B', ' '},
        {' ', ' ', 'H', ' ', 'G'},
        {' ', ' ', ' ', ' ', ' '}
    };

    initialize_game(&game, 4, 5, initial_grid);

    GameStatus status;
    status = move_player(&game, 1, 1); // Attempt to move to (1, 1)
    if (status != GAME_CONTINUE) {
        print_game_status(status);
    }

    // Additional game logic would go here...

    return 0;
}
