#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NAME_LENGTH 21
#define MAX_GROUPS 100
#define MAX_NAMES 100

typedef struct {
    char names[MAX_NAMES][MAX_NAME_LENGTH];
    int count;
} Group;

int is_valid_group(Group *group, int *used, int n) {
    // Check if the group can be formed without reusing names
    for (int i = 0; i < group->count; i++) {
        for (int j = 0; j < n; j++) {
            if (strcmp(group->names[i], group->names[j]) == 0 && used[j]) {
                return 0; // Name already used
            }
        }
    }
    return 1; // Valid group
}

void mark_used(Group *group, int *used, int n) {
    // Mark names in the group as used
    for (int i = 0; i < group->count; i++) {
        for (int j = 0; j < n; j++) {
            if (strcmp(group->names[i], group->names[j]) == 0) {
                used[j] = 1; // Mark this name as used
            }
        }
    }
}

int main() {
    int n, m;
    while (scanf("%d %d", &n, &m) == 2 && n > 0 && m > 0) {
        char names[MAX_NAMES][MAX_NAME_LENGTH];
        Group groups[MAX_GROUPS];
        int used[MAX_NAMES] = {0}; // Track used names
        int valid_group_count = 0;

        // Read names
        for (int i = 0; i < n; i++) {
            scanf("%s", names[i]);
        }

        // Read groups
        for (int i = 0; i < m; i++) {
            int j;
            scanf("%d", &j);
            groups[i].count = j;
            for (int k = 0; k < j; k++) {
                scanf("%s", groups[i].names[k]);
            }
        }

        // Check for valid groups
        for (int i = 0; i < m; i++) {
            if (is_valid_group(&groups[i], used, n)) {
                mark_used(&groups[i], used, n);
                valid_group_count++;
            }
        }

        // Output results
        if (valid_group_count > 0) {
            printf("%d\n", valid_group_count);
            for (int i = 0; i < m; i++) {
                if (is_valid_group(&groups[i], used, n)) {
                    for (int k = 0; k < groups[i].count; k++) {
                        printf("%s ", groups[i].names[k]);
                    }
                    printf("\n");
                }
            }
        } else {
            printf("Impossible to form groups\n");
        }
    }
    return 0;
}
