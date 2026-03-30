#include <stdio.h>
#include <stdlib.h>

#define MAX_DESKS 100
#define MAX_CAPACITY 100
#define MAX_BOOKS 100

typedef struct {
    int book_id;
    int last_used; // To track the least recently used book
} Book;

typedef struct {
    Book books[MAX_CAPACITY];
    int count; // Current number of books on the desk
} Desk;

void process_requests(int m, int c, int n, int requests[][MAX_BOOKS]) {
    Desk desks[MAX_DESKS] = {0}; // Initialize desks
    int total_assignments = 0;

    for (int i = 0; i < n; i++) {
        int *request = requests[i];
        for (int j = 0; request[j] != -1; j++) { // Assuming -1 marks the end of requests
            int book_id = request[j];
            int assigned = 0;

            // Try to assign the book to an existing desk
            for (int d = 0; d < m; d++) {
                if (desks[d].count < c) { // If desk has capacity
                    desks[d].books[desks[d].count].book_id = book_id;
                    desks[d].books[desks[d].count].last_used = i; // Mark as recently used
                    desks[d].count++;
                    total_assignments++;
                    assigned = 1;
                    break;
                }
            }

            // If not assigned, we need to replace a book using LRU policy
            if (!assigned) {
                int lru_desk = -1;
                int lru_index = -1;
                int lru_time = n; // Start with a time greater than any request

                // Find the least recently used book across all desks
                for (int d = 0; d < m; d++) {
                    if (desks[d].count > 0) {
                        for (int k = 0; k < desks[d].count; k++) {
                            if (desks[d].books[k].last_used < lru_time) {
                                lru_time = desks[d].books[k].last_used;
                                lru_desk = d;
                                lru_index = k;
                            }
                        }
                    }
                }

                // Replace the LRU book
                if (lru_desk != -1) {
                    desks[lru_desk].books[lru_index].book_id = book_id;
                    desks[lru_desk].books[lru_index].last_used = i; // Update last used time
                    total_assignments++;
                }
            }
        }
    }

    printf("%d\n", total_assignments); // Output the total number of assignments
}

int main() {
    int m, c, n;

    // Read number of desks, capacity, and number of students
    while (scanf("%d %d %d", &m, &c, &n) == 3) {
        if (m <= 0 || c <= 0 || n <= 0) {
            return 0; // Terminate gracefully on non-positive values
        }

        int requests[MAX_BOOKS][MAX_BOOKS] = {0}; // Store requests
        for (int i = 0; i < n; i++) {
            int j = 0;
            while (scanf("%d", &requests[i][j]) == 1 && requests[i][j] != -1) {
                j++;
            }
            requests[i][j] = -1; // Mark end of requests
        }

        process_requests(m, c, n, requests); // Process the requests
    }

    return 0;
}
