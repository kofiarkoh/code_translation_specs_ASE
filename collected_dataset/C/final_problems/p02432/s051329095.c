/*
 * ID: ITP2_1_B
 * Problem: Deque
 */

#include <stdio.h>

int main(void)
{
    int a[400000];
    int n_query;
    int last = 0;
    int i;

    scanf("%d", &n_query);
    for (i = 0; i < n_query; i++) {
        int query, index, value;
        int j;
        scanf("%d %d", &query, &index);
        switch (query) {
            case 0: // push(index, value)
                scanf("%d", &value);
                if (index == 0) {
                    for (j = last; j >= 0; j--)
                        a[j + 1] = a[j];
                    a[0] = value;
                    last++;
                } else {
                    a[last++] = value;
                }
                break;
            case 1: // randomAccess(index)
                printf("%d\n", a[index]);
                break;
            case 2: // pop(index)
                if (index == 0) {
                    for (j = 1; j <= last; j++)
                        a[j - 1] = a[j];
                    last--;
                } else {
                    last--;
                }
                break;
        }
    }

    return 0;
}
