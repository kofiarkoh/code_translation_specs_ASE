#include <stdio.h>
#include <stdlib.h>

typedef struct Pool {
    int area;
    int x_coordinate;
    struct Pool* next;
} Pool;

typedef struct Stack {
    int* positions;
    int top;
    int capacity;
} Stack;

// Function to create a stack
Stack* createStack(int capacity) {
    Stack* stack = (Stack*)malloc(sizeof(Stack));
    stack->capacity = capacity;
    stack->top = -1;
    stack->positions = (int*)malloc(capacity * sizeof(int));
    return stack;
}

// Function to push an element onto the stack
void push(Stack* stack, int position) {
    if (stack->top < stack->capacity - 1) {
        stack->positions[++stack->top] = position;
    }
}

// Function to pop an element from the stack
int pop(Stack* stack) {
    if (stack->top >= 0) {
        return stack->positions[stack->top--];
    }
    return -1; // Indicate empty stack
}

// Function to free the stack
void freeStack(Stack* stack) {
    free(stack->positions);
    free(stack);
}

// Function to add a pool to the linked list
Pool* addPool(Pool* head, int area, int x_coordinate) {
    Pool* newPool = (Pool*)malloc(sizeof(Pool));
    newPool->area = area;
    newPool->x_coordinate = x_coordinate;
    newPool->next = head;
    return newPool;
}

// Function to free the pool linked list
void freePools(Pool* head) {
    Pool* temp;
    while (head) {
        temp = head;
        head = head->next;
        free(temp);
    }
}

int main() {
    char input[1000]; // Assuming input will not exceed 999 characters
    fgets(input, sizeof(input), stdin); // Read input from stdin

    Stack* stack = createStack(100); // Create a stack with a capacity of 100
    Pool* pools = NULL;
    int totalArea = 0;
    int poolCount = 0;
    int currentPosition = 0;

    for (int i = 0; input[i] != '\0'; i++) {
        char ch = input[i];
        if (ch == '\\') {
            // Push current position onto the stack
            push(stack, currentPosition);
            currentPosition = i; // Update current position to the backslash position
        } else if (ch == '/') {
            // Attempt to form a pool
            if (stack->top != -1) {
                int start = pop(stack); // Get the last position from the stack
                int area = currentPosition - start; // Calculate area
                if (area > 0) { // Only consider positive areas
                    pools = addPool(pools, area, start);
                    totalArea += area;
                    poolCount++;
                }
            }
        } else if (ch != '_') {
            // Update current position for other characters
            currentPosition = i;
        }
    }

    // Output total area and count of pools
    printf("Total Area: %d\n", totalArea);
    printf("Pool Count: %d\n", poolCount);

    // Output individual pool areas in reverse order
    Pool* current = pools;
    while (current) {
        printf("Pool Area: %d at x-coordinate: %d\n", current->area, current->x_coordinate);
        current = current->next;
    }

    // Free allocated memory
    freePools(pools);
    freeStack(stack);

    return 0;
}
