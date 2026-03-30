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

typedef struct {
    Group groups[MAX_GROUPS];
    int group_count;
} TestCase;

int is_valid_combination(int *selected, int group_count, TestCase *test_case) {
    int used[MAX_NAMES] = {0}; // Track used names
    for (int i = 0; i < group_count; i++) {
        if (selected[i]) {
            for (int j = 0; j < test_case->groups[i].count; j++) {
                int name_index = -1;
                for (int k = 0; k < test_case->groups[i].count; k++) {
                    if (strcmp(test_case->groups[i].names[j], test_case->groups[i].names[k]) == 0) {
                        name_index = k;
                        break;
                    }
                }
                if (name_index != -1) {
                    if (used[name_index]) {
                        return 0; // Name already used
                    }
                    used[name_index] = 1; // Mark name as used
                }
            }
        }
    }
    return 1; // Valid combination
}

void find_max_groups(TestCase *test_case) {
    int max_groups = 0;
    int best_combination[MAX_GROUPS] = {0};

    int total_combinations = 1 << test_case->group_count; // 2^group_count
    for (int i = 1; i < total_combinations; i++) {
        int selected[MAX_GROUPS] = {0};
        int group_count = 0;

        for (int j = 0; j < test_case->group_count; j++) {
            if (i & (1 << j)) {
                selected[j] = 1;
                group_count++;
            }
        }

        if (is_valid_combination(selected, test_case->group_count, test_case)) {
            if (group_count > max_groups) {
                max_groups = group_count;
                memcpy(best_combination, selected, sizeof(selected));
            }
        }
    }

    if (max_groups == 0) {
        printf("Impossible to form groups\n");
    } else {
        printf("Maximum distinct groups: %d\n", max_groups);
        for (int i = 0; i < test_case->group_count; i++) {
            if (best_combination[i]) {
                for (int j = 0; j < test_case->groups[i].count; j++) {
                    printf("%s ", test_case->groups[i].names[j]);
                }
                printf("\n");
            }
        }
    }
}

int main() {
    int n, m;
    TestCase test_case;

    // Read number of names and groups
    if (scanf("%d %d", &n, &m) != 2 || n <= 0 || m <= 0) {
        printf("Invalid input\n");
        return 1; // Invalid input handling
    }

    test_case.group_count = m;

    // Read names
    for (int i = 0; i < n; i++) {
        scanf("%s", test_case.groups[0].names[i]); // Store names in the first group for simplicity
    }

    // Read groups
    for (int i = 0; i < m; i++) {
        int j;
        scanf("%d", &j);
        test_case.groups[i].count = j;
        for (int k = 0; k < j; k++) {
            scanf("%s", test_case.groups[i].names[k]);
        }
    }

    find_max_groups(&test_case);
    return 0;
}
