/*
 * ID: 3041
 * Problem: round and round
 */

#include <stdio.h>

int main(void)
{
    int n, query;
    scanf("%d %d", &n, &query);

    int index = 0, value;
    int i;
    for (i = 0; i < query; i++) {
        int command, k;
        scanf("%d %d", &command, &k);

        switch (command) {
            case 0:
                value = (index + k) % n;
                printf("%d\n", value ? value : n);
                break;
            case 1:
                index = (index + k) % n;
                break;
        }
    }

    return 0;
}

