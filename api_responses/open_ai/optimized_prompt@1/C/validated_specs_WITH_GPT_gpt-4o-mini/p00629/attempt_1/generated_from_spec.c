#include <stdio.h>
#include <stdlib.h>

#define MAX_RECORDS 300
#define MAX_IDENTIFIERS 25

typedef struct {
    int attributes[4]; // Array to hold the four attributes of a record
    int identifier;    // Identifier for the record, based on its position
} Record;

// Comparison function for sorting records
int compare_records(const void *a, const void *b) {
    Record *recordA = (Record *)a;
    Record *recordB = (Record *)b;

    // Sort by third attribute (descending), then fourth (ascending), then first (ascending)
    if (recordA->attributes[2] != recordB->attributes[2]) {
        return recordB->attributes[2] - recordA->attributes[2]; // Descending
    }
    if (recordA->attributes[3] != recordB->attributes[3]) {
        return recordA->attributes[3] - recordB->attributes[3]; // Ascending
    }
    return recordA->attributes[0] - recordB->attributes[0]; // Ascending
}

int main() {
    int n;
    while (1) {
        // Read the number of records
        scanf("%d", &n);
        if (n == 0) {
            break; // Terminate on receiving 0
        }

        if (n > MAX_RECORDS) {
            n = MAX_RECORDS; // Limit to MAX_RECORDS if input exceeds
        }

        Record records[MAX_RECORDS];
        int frequency[1001] = {0}; // Assuming second attribute is in range 0-1000
        int unique_count = 0;

        // Read records
        for (int i = 0; i < n; i++) {
            records[i].identifier = i; // Assign identifier based on position
            scanf("%d %d %d %d", &records[i].attributes[0], 
                                  &records[i].attributes[1], 
                                  &records[i].attributes[2], 
                                  &records[i].attributes[3]);
            frequency[records[i].attributes[1]]++; // Count frequency of second attribute
        }

        // Sort records based on the specified criteria
        qsort(records, n, sizeof(Record), compare_records);

        // Output identifiers based on frequency criteria
        int output_count = 0;
        for (int i = 0; i < n && output_count < MAX_IDENTIFIERS; i++) {
            int second_attr = records[i].attributes[1];
            if (frequency[second_attr] > 0) {
                printf("%d\n", records[i].identifier);
                output_count++;
                frequency[second_attr]--; // Decrease frequency to limit output
            }
        }
    }
    return 0;
}
