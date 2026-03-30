#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int data;
    struct Node* prev;
    struct Node* next;
} Node;

typedef struct List {
    Node* head;
    Node* tail;
    Node* cursor; // Pointer to the current position in the list
} List;

// Function to create a new node
Node* create_node(int data) {
    Node* new_node = (Node*)malloc(sizeof(Node));
    new_node->data = data;
    new_node->prev = NULL;
    new_node->next = NULL;
    return new_node;
}

// Function to initialize the list with a sentinel node
List* initialize_list(int sentinel_value) {
    List* list = (List*)malloc(sizeof(List));
    Node* sentinel = create_node(sentinel_value);
    list->head = sentinel;
    list->tail = sentinel;
    list->cursor = sentinel; // Cursor starts at the sentinel
    return list;
}

// Function to insert a new node after the cursor
void insert(List* list, int value) {
    Node* new_node = create_node(value);
    new_node->prev = list->cursor;
    new_node->next = list->cursor->next;

    if (list->cursor->next != NULL) {
        list->cursor->next->prev = new_node;
    } else {
        list->tail = new_node; // Update tail if inserting at the end
    }
    list->cursor->next = new_node;
}

// Function to move the cursor forward
void move_forward(List* list) {
    if (list->cursor->next == NULL) {
        fprintf(stderr, "Error: Attempt to move beyond the end of the list.\n");
        exit(EXIT_FAILURE);
    }
    list->cursor = list->cursor->next;
}

// Function to move the cursor backward
void move_backward(List* list) {
    if (list->cursor->prev == NULL) {
        fprintf(stderr, "Error: Attempt to move beyond the front of the list.\n");
        exit(EXIT_FAILURE);
    }
    list->cursor = list->cursor->prev;
}

// Function to erase the node after the cursor
void erase(List* list) {
    if (list->cursor->next == NULL) {
        return; // Nothing to erase
    }
    Node* to_delete = list->cursor->next;
    list->cursor->next = to_delete->next;

    if (to_delete->next != NULL) {
        to_delete->next->prev = list->cursor;
    } else {
        list->tail = list->cursor; // Update tail if erasing the last node
    }
    free(to_delete);
}

// Function to print the list contents
void print_list(List* list) {
    Node* current = list->head;
    while (current != NULL) {
        if (current != list->head) { // Exclude the sentinel node
            printf("%d ", current->data);
        }
        current = current->next;
    }
    printf("\n");
}

// Main function to handle input and execute commands
int main() {
    int num_operations;
    scanf("%d", &num_operations);

    List* list = initialize_list(0); // Initialize with sentinel value 0

    for (int i = 0; i < num_operations; i++) {
        char command[10];
        int value;
        scanf("%s", command);

        if (strcmp(command, "insert") == 0) {
            scanf("%d", &value);
            insert(list, value);
        } else if (strcmp(command, "move") == 0) {
            scanf("%s", command); // Read direction
            if (strcmp(command, "forward") == 0) {
                move_forward(list);
            } else if (strcmp(command, "backward") == 0) {
                move_backward(list);
            }
        } else if (strcmp(command, "erase") == 0) {
            erase(list);
        }
    }

    print_list(list);

    // Free memory (not shown for brevity, but should be implemented)
    // Free all nodes and the list itself

    return 0;
}
