#include <stdio.h>
#include <stdlib.h>

#define MAX_GRID_SIZE 100
#define MAX_MOVES 20

typedef struct {
    int x, y;
} Position;

typedef struct {
    Position pos;
    int movesA, movesB;
} State;

char grid[MAX_GRID_SIZE][MAX_GRID_SIZE];
int H, W, A, B;

int is_valid(int x, int y) {
    return (x >= 0 && x < H && y >= 0 && y < W && grid[x][y] != '#');
}

int bfs(Position start, Position goal) {
    // Check immediate failure conditions
    if (start.x == H - 1 || (start.x + 1 < H && grid[start.x + 1][start.y] == '#')) {
        return -1;
    }

    // BFS setup
    State queue[MAX_GRID_SIZE * MAX_GRID_SIZE];
    int front = 0, rear = 0;
    int visited[MAX_GRID_SIZE][MAX_GRID_SIZE][A + 1][B + 1] = {0};

    queue[rear++] = (State){start, 0, 0};
    visited[start.x][start.y][0][0] = 1;

    while (front < rear) {
        State current = queue[front++];
        Position currPos = current.pos;

        // Check if we reached the goal
        if (currPos.x == goal.x && currPos.y == goal.y) {
            return (current.movesA + current.movesB);
        }

        // Explore neighbors
        Position directions[4] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}}; // Up, Down, Left, Right
        for (int i = 0; i < 4; i++) {
            Position nextPos = {currPos.x + directions[i].x, currPos.y + directions[i].y};

            if (is_valid(nextPos.x, nextPos.y)) {
                // Determine move type (A or B) based on some criteria (not specified in the spec)
                int moveType = (i % 2 == 0) ? 0 : 1; // Example: Up/Down as A, Left/Right as B
                int newMovesA = current.movesA + (moveType == 0);
                int newMovesB = current.movesB + (moveType == 1);

                // Check move limits
                if (newMovesA <= A && newMovesB <= B && !visited[nextPos.x][nextPos.y][newMovesA][newMovesB]) {
                    visited[nextPos.x][nextPos.y][newMovesA][newMovesB] = 1;
                    queue[rear++] = (State){nextPos, newMovesA, newMovesB};
                }
            }
        }
    }

    return -1; // Goal not reachable
}

int main() {
    // Input reading (assumed to be provided correctly)
    scanf("%d %d %d %d", &H, &W, &A, &B);
    Position start, goal;

    for (int i = 0; i < H; i++) {
        scanf("%s", grid[i]);
        for (int j = 0; j < W; j++) {
            if (grid[i][j] == 'S') {
                start = (Position){i, j};
            } else if (grid[i][j] == 'G') {
                goal = (Position){i, j};
            }
        }
    }

    int result = bfs(start, goal);
    if (result > MAX_MOVES) {
        result = -1; // Exceeds threshold
    }
    printf("%d\n", result);
    return 0;
}
