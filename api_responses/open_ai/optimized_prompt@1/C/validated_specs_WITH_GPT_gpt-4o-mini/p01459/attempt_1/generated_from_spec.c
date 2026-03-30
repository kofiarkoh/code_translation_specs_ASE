#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
int H, W;
int maxMovesA, maxMovesB;
int visited[MAX_GRID_SIZE][MAX_GRID_SIZE][MAX_MOVES + 1][MAX_MOVES + 1];

int isValid(int x, int y) {
    return (x >= 0 && x < H && y >= 0 && y < W && grid[x][y] != '#');
}

int bfs(Position start, Position goal) {
    if (start.x == H - 1 || 
        (start.x < H - 1 && (grid[start.x + 1][start.y] == '#' || 
        (start.y > 0 && grid[start.x][start.y - 1] == '#') || 
        (start.y < W - 1 && grid[start.x][start.y + 1] == '#')))) {
        return -1; // Immediate failure condition
    }

    State queue[MAX_GRID_SIZE * MAX_GRID_SIZE * (MAX_MOVES + 1) * (MAX_MOVES + 1)];
    int front = 0, rear = 0;

    queue[rear++] = (State){start, 0, 0};
    visited[start.x][start.y][0][0] = 1;

    while (front < rear) {
        State current = queue[front++];
        
        if (current.pos.x == goal.x && current.pos.y == goal.y) {
            return current.movesA + current.movesB; // Return total moves
        }

        // Explore neighbors
        Position directions[4] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}}; // Down, Right, Up, Left
        for (int i = 0; i < 4; i++) {
            Position next = {current.pos.x + directions[i].x, current.pos.y + directions[i].y};

            if (isValid(next.x, next.y)) {
                // Move A
                if (current.movesA < maxMovesA && !visited[next.x][next.y][current.movesA + 1][current.movesB]) {
                    visited[next.x][next.y][current.movesA + 1][current.movesB] = 1;
                    queue[rear++] = (State){next, current.movesA + 1, current.movesB};
                }
                // Move B
                if (current.movesB < maxMovesB && !visited[next.x][next.y][current.movesA][current.movesB + 1]) {
                    visited[next.x][next.y][current.movesA][current.movesB + 1] = 1;
                    queue[rear++] = (State){next, current.movesA, current.movesB + 1};
                }
            }
        }
    }

    return -1; // Goal not reachable
}

int main() {
    // Input reading
    scanf("%d %d %d %d", &H, &W, &maxMovesA, &maxMovesB);
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

    // Call BFS to find the minimum moves
    int result = bfs(start, goal);
    if (result > MAX_MOVES) {
        result = -1; // Exceeding threshold
    }

    printf("%d\n", result);
    return 0;
}
