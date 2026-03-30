#include <stdio.h>
#include <stdlib.h>

// Define the structure for a node in the doubly linked list
typedef struct Node {
    int data;
    struct Node* next;
    struct Node* prev;
} Node;

// Define the structure for the doubly linked list
typedef struct LinkedList {
    Node* head;
    Node* tail;
    Node* cursor; // Pointer to the current position in the list
} LinkedList;

// Function to create a new node
Node* createNode(int data) {
    Node* newNode = (Node*)malloc(sizeof(Node));
    newNode->data = data;
    newNode->next = NULL;
    newNode->prev = NULL;
    return newNode;
}

// Function to initialize the linked list with a sentinel node
LinkedList* initializeList() {
    LinkedList* list = (LinkedList*)malloc(sizeof(LinkedList));
    list->head = createNode(0); // Sentinel node with value 0
    list->tail = list->head; // Initially, head and tail are the same
    list->cursor = list->head; // Cursor starts at the sentinel
    return list;
}

// Function to insert a new node after the cursor
void insertNode(LinkedList* list, int value) {
    Node* newNode = createNode(value);
    newNode->next = list->cursor->next;
    newNode->prev = list->cursor;

    if (list->cursor->next != NULL) {
        list->cursor->next->prev = newNode;
    } else {
        list->tail = newNode; // Update tail if inserting at the end
    }
    list->cursor->next = newNode;
}

// Function to move the cursor forward
void moveCursorForward(LinkedList* list) {
    if (list->cursor->next == NULL) {
        fprintf(stderr, "Error: Attempt to move cursor beyond the end of the list.\n");
        exit(EXIT_FAILURE);
    }
    list->cursor = list->cursor->next;
}

// Function to move the cursor backward
void moveCursorBackward(LinkedList* list) {
    if (list->cursor->prev == NULL) {
        fprintf(stderr, "Error: Attempt to move cursor beyond the front of the list.\n");
        exit(EXIT_FAILURE);
    }
    list->cursor = list->cursor->prev;
}

// Function to erase the node at the cursor
void eraseNode(LinkedList* list) {
    if (list->cursor == list->head) {
        fprintf(stderr, "Error: Cannot erase the sentinel node.\n");
        return; // Do not erase the sentinel node
    }
    Node* toDelete = list->cursor;
    if (toDelete->prev != NULL) {
        toDelete->prev->next = toDelete->next;
    }
    if (toDelete->next != NULL) {
        toDelete->next->prev = toDelete->prev;
    }
    if (toDelete == list->tail) {
        list->tail = toDelete->prev; // Update tail if necessary
    }
    list->cursor = toDelete->prev; // Move cursor back after deletion
    free(toDelete); // Free the memory of the deleted node
}

// Function to print the contents of the list
void printList(LinkedList* list) {
    Node* current = list->head->next; // Skip the sentinel node
    while (current != NULL) {
        printf("%d ", current->data);
        current = current->next;
    }
    printf("\n");
}

// Main function to handle input and execute operations
int main() {
    int numOperations;
    scanf("%d", &numOperations); // Read the number of operations

    LinkedList* list = initializeList(); // Initialize the linked list

    for (int i = 0; i < numOperations; i++) {
        char command[10];
        int value;
        scanf("%s", command); // Read the command

        if (strcmp(command, "insert") == 0) {
            scanf("%d", &value); // Read the value for insertion
            insertNode(list, value);
        } else if (strcmp(command, "move") == 0) {
            scanf("%s", command); // Read direction (forward/backward)
            if (strcmp(command, "forward") == 0) {
                moveCursorForward(list);
            } else if (strcmp(command, "backward") == 0) {
                moveCursorBackward(list);
            }
        } else if (strcmp(command, "erase") == 0) {
            eraseNode(list);
        }
    }

    printList(list); // Print the final contents of the list

    // Free the allocated memory for the list
    Node* current = list->head;
    while (current != NULL) {
        Node* next = current->next;
        free(current);
        current = next;
    }
    free(list); // Free the list structure

    return 0;
}
