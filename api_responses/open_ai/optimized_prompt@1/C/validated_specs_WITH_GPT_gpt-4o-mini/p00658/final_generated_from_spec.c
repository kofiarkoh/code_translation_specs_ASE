#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
        return; // Graceful termination for zero dimensions
    }
    game->height = height;
    game->width = width;
    memcpy(game->grid, grid, height * width * sizeof(char));
    
    // Find the player's initial position
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == 'P') { // Assuming 'P' represents the player
                game->player_x = j;
                game->player_y = i;
                return;
            }
        }
    }
}

int is_within_bounds(int x, int y, Game *game) {
    return x >= 0 && x < game->width && y >= 0 && y < game->height;
}

GameStatus check_game_status(Game *game) {
    // Check for game-over conditions
    if (game->grid[game->player_y][game->player_x] == 'G') { // Assuming 'G' is the goal
        return GAME_CLEARED;
    }
    if (game->grid[game->player_y][game->player_x] == 'H') { // Assuming 'H' is a hole
        return GAME_DEATH_HOLE;
    }
    // Additional checks for blocks or other conditions can be added here
    return GAME_CONTINUE;
}

void move_player(Game *game, int new_x, int new_y) {
    if (is_within_bounds(new_x, new_y, game)) {
        game->player_x = new_x;
        game->player_y = new_y;
    }
}

GameStatus process_command(Game *game, char command) {
    int new_x = game->player_x;
    int new_y = game->player_y;

    switch (command) {
        case 'W': // Move up
            new_y--;
            break;
        case 'S': // Move down
            new_y++;
            break;
        case 'A': // Move left
            new_x--;
            break;
        case 'D': // Move right
            new_x++;
            break;
        default:
            return GAME_CONTINUE; // Invalid command, ignore
    }

    move_player(game, new_x, new_y);
    return check_game_status(game);
}

void game_loop(Game *game) {
    char command;
    GameStatus status = GAME_CONTINUE;

    while (status == GAME_CONTINUE) {
        printf("Enter command (W/A/S/D): ");
        scanf(" %c", &command); // Read command

        status = process_command(game, command);
    }

    // Output game status
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
            printf("Gave Up!\n");
            break;
        default:
            break;
    }
}

int main() {
    Game game;
    char grid[MAX_HEIGHT][MAX_WIDTH] = {
        {' ', ' ', ' ', ' ', 'G'},
        {' ', 'P', ' ', 'H', ' '},
        {' ', ' ', ' ', ' ', ' '}
    };

    initialize_game(&game, 3, 5, grid);
    game_loop(&game);

    return 0;
}
