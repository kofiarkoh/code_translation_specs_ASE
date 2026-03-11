#include <stdio.h>
#include <stdlib.h>


int main () {
    int n, i, j, x, l, longest;
    int *a, *len;

    scanf("%d", &n);
    a = (int*)malloc(sizeof(int) * (n + 1));
    len = (int*)malloc(sizeof(int) * (n + 1));

    a[0] = 0;
    for (i = 1; i <= n; i++) {
        scanf("%d", &a[i]);
    }

    len[0] = 0;
    for (i = 1; i <= n; i++) {
        x = a[i];
        l = 1;
        for (j = 0; j < i; j++) {
            if (a[j] < x && len[j] >= l) {
                l = len[j] + 1;
            }
        }
        len[i] = l;
    }

    longest = 1;
    for (i = 1; i <= n; i++) {
        if (longest < len[i]) {
            longest = len[i];
        }
    }

    printf("%d\n", longest);

    return 0;
}