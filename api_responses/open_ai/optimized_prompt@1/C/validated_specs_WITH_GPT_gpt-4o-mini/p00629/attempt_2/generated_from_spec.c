#include <stdio.h>
#include <stdlib.h>

#define MAX_RECORDS 300
#define MAX_OUTPUT 25

typedef struct {
    int I; // Identifier
    int U; // Second attribute
    int A; // Third attribute
    int P; // Fourth attribute
} Record;

// Comparison function for sorting
int compare(const void *a, const void *b) {
    Record *recordA = (Record *)a;
    Record *recordB = (Record *)b;

    // Sort by A (descending), then by P (ascending), then by I (ascending)
    if (recordA->A != recordB->A) {
        return recordB->A - recordA->A; // Descending order for A
    }
    if (recordA->P != recordB->P) {
        return recordA->P - recordB->P; // Ascending order for P
    }
    return recordA->I - recordB->I; // Ascending order for I
}

int main() {
    int n;
    Record records[MAX_RECORDS];
    int output[MAX_OUTPUT];
    int outputCount;

    while (1) {
        // Read the number of records
        scanf("%d", &n);
        if (n == 0) {
            break; // Terminate on input of 0
        }

        // Read records
        for (int i = 0; i < n && i < MAX_RECORDS; i++) {
            scanf("%d %d %d %d", &records[i].I, &records[i].U, &records[i].A, &records[i].P);
        }

        // Sort records
        qsort(records, n < MAX_RECORDS ? n : MAX_RECORDS, sizeof(Record), compare);

        // Frequency counting for U
        int frequency[300] = {0}; // Assuming U values are in a reasonable range
        for (int i = 0; i < n && i < MAX_RECORDS; i++) {
            frequency[records[i].U]++;
        }

        // Select identifiers based on frequency criteria
        outputCount = 0;
        for (int i = 0; i < n && outputCount < MAX_OUTPUT; i++) {
            int u = records[i].U;
            if (frequency[u] == 3 && outputCount < 3) {
                output[outputCount++] = records[i].I;
                frequency[u] = 0; // Mark as selected
            } else if (frequency[u] == 2 && outputCount < 2) {
                output[outputCount++] = records[i].I;
                frequency[u] = 0; // Mark as selected
            } else if (frequency[u] == 1 && outputCount < 1) {
                output[outputCount++] = records[i].I;
                frequency[u] = 0; // Mark as selected
            }
        }

        // Print output identifiers
        for (int i = 0; i < outputCount; i++) {
            printf("%d\n", output[i]);
        }
    }

    return 0;
}
