#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NAMES 100
#define MAX_NAME_LENGTH 21
#define MAX_GROUPS 10

typedef struct {
    char names[MAX_NAMES][MAX_NAME_LENGTH];
    int count;
} Group;

int is_valid_group(Group *groups, int group_count, Group *new_group) {
    // Check if the new group overlaps with any existing groups
    for (int i = 0; i < group_count; i++) {
        for (int j = 0; j < new_group->count; j++) {
            for (int k = 0; k < groups[i].count; k++) {
                if (strcmp(groups[i].names[k], new_group->names[j]) == 0) {
                    return 0; // Overlap found
                }
            }
        }
    }
    return 1; // No overlap
}

void print_groups(Group *groups, int group_count) {
    for (int i = 0; i < group_count; i++) {
        printf("Group %d: ", i + 1);
        for (int j = 0; j < groups[i].count; j++) {
            printf("%s ", groups[i].names[j]);
        }
        printf("\n");
    }
}

int main() {
    int n, m;
    while (scanf("%d %d", &n, &m) == 2 && n > 0 && m > 0) {
        char names[MAX_NAMES][MAX_NAME_LENGTH];
        Group groups[MAX_GROUPS];
        int group_count = 0;

        // Read names
        for (int i = 0; i < n; i++) {
            scanf("%s", names[i]);
        }

        // Read groups
        for (int i = 0; i < m; i++) {
            int j;
            scanf("%d", &j);
            if (j > MAX_NAMES) {
                continue; // Ignore groups that exceed max names
            }
            Group new_group;
            new_group.count = j;
            for (int k = 0; k < j; k++) {
                scanf("%s", new_group.names[k]);
            }

            // Check if the new group can be added
            if (is_valid_group(groups, group_count, &new_group)) {
                groups[group_count++] = new_group; // Add new group
            }
        }

        // Output results
        if (group_count > 0) {
            printf("Maximum distinct groups: %d\n", group_count);
            print_groups(groups, group_count);
        } else {
            printf("Impossible to form groups.\n");
        }
    }
    return 0;
}
