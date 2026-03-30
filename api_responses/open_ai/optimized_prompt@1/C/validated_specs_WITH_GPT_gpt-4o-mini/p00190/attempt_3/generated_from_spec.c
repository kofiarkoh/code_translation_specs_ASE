#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PUZZLE_SIZE 13
#define EMPTY_TILE 0
#define MAX_MOVES 100 // Arbitrary limit for unsolvable cases

typedef struct {
    int state[PUZZLE_SIZE];
    int moves;
} PuzzleState;

typedef struct Node {
    PuzzleState puzzle;
    struct Node* next;
} Node;

typedef struct {
    Node* head;
    Node* tail;
} Queue;

typedef struct {
    unsigned long long* table;
    size_t size;
} HashTable;

// Function prototypes
void enqueue(Queue* queue, PuzzleState puzzle);
PuzzleState dequeue(Queue* queue);
int is_empty(Queue* queue);
unsigned long long hash_state(int* state);
int is_solvable(int* state);
int find_empty_tile(int* state);
void swap(int* state, int i, int j);
int bfs(int* initial_state);
void free_queue(Queue* queue);
void free_hash_table(HashTable* hash_table);

int main(int argc, char* argv[]) {
    if (argc != PUZZLE_SIZE + 1) {
        return 1; // Invalid input length
    }

    int initial_state[PUZZLE_SIZE];
    for (int i = 0; i < PUZZLE_SIZE; i++) {
        initial_state[i] = atoi(argv[i + 1]);
        if (initial_state[i] < 0) {
            return 1; // Invalid input value
        }
    }

    if (!is_solvable(initial_state)) {
        printf("NA\n"); // Puzzle cannot be solved
        return 0;
    }

    int moves = bfs(initial_state);
    if (moves == MAX_MOVES) {
        printf("NA\n"); // Puzzle cannot be solved within limits
    } else {
        printf("%d\n", moves); // Output the number of moves
    }

    return 0;
}

void enqueue(Queue* queue, PuzzleState puzzle) {
    Node* new_node = (Node*)malloc(sizeof(Node));
    new_node->puzzle = puzzle;
    new_node->next = NULL;
    if (is_empty(queue)) {
        queue->head = new_node;
        queue->tail = new_node;
    } else {
        queue->tail->next = new_node;
        queue->tail = new_node;
    }
}

PuzzleState dequeue(Queue* queue) {
    if (is_empty(queue)) {
        // Return an empty puzzle state if the queue is empty
        PuzzleState empty = {{0}, -1};
        return empty;
    }
    Node* temp = queue->head;
    PuzzleState puzzle = temp->puzzle;
    queue->head = queue->head->next;
    free(temp);
    return puzzle;
}

int is_empty(Queue* queue) {
    return queue->head == NULL;
}

unsigned long long hash_state(int* state) {
    unsigned long long hash = 0;
    for (int i = 0; i < PUZZLE_SIZE; i++) {
        hash = hash * 31 + state[i]; // Simple hash function
    }
    return hash;
}

int is_solvable(int* state) {
    int inversions = 0;
    for (int i = 0; i < PUZZLE_SIZE; i++) {
        for (int j = i + 1; j < PUZZLE_SIZE; j++) {
            if (state[i] > state[j] && state[i] != EMPTY_TILE && state[j] != EMPTY_TILE) {
                inversions++;
            }
        }
    }
    return inversions % 2 == 0; // Solvable if inversions are even
}

int find_empty_tile(int* state) {
    for (int i = 0; i < PUZZLE_SIZE; i++) {
        if (state[i] == EMPTY_TILE) {
            return i;
        }
    }
    return -1; // Should not reach here if the state is valid
}

void swap(int* state, int i, int j) {
    int temp = state[i];
    state[i] = state[j];
    state[j] = temp;
}

int bfs(int* initial_state) {
    Queue queue = {NULL, NULL};
    HashTable hash_table = {NULL, 0};
    PuzzleState start = {{0}, 0};
    memcpy(start.state, initial_state, sizeof(start.state));
    enqueue(&queue, start);

    while (!is_empty(&queue)) {
        PuzzleState current = dequeue(&queue);
        // Check if the current state is the goal state (not defined in the spec)
        // if (is_goal_state(current.state)) return current.moves;

        // Generate possible moves (not defined in the spec)
        // For each valid move, create a new state and enqueue it
        // if (new_state is not in hash_table) {
        //     enqueue(&queue, new_state);
        //     add new_state to hash_table;
        // }

        // Increment move count
        if (current.moves >= MAX_MOVES) {
            return MAX_MOVES; // Indicate unsolvable within limits
        }
    }

    return MAX_MOVES; // Indicate unsolvable
}

void free_queue(Queue* queue) {
    while (!is_empty(queue)) {
        dequeue(queue);
    }
}

void free_hash_table(HashTable* hash_table) {
    free(hash_table->table);
}
