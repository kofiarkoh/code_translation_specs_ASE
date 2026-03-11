#include <stdio.h>
#include <stdlib.h>

int main (void) {
    int n, q, i, j, k, l;
    scanf("%d %d", &n, &q);
    long int sum = (long int) (n - 2) * (n - 2);
    int r[n - 2], d[n - 2];
    for (i = 0; i < n - 2; i++) {
        r[i] = n - 2;
        d[i] = n - 2;
    }

    for (i = 0; i < q; i++) {
        scanf("%d %d", &k, &l);
        if (k == 1) {
            for (j = 0; j < d[l - 2]; j++)
                if (r[j] > l - 2) r[j] = l - 2;
        } else {
            for (j = 0; j < r[l - 1]; j++) 
                if (d[j] > l - 2) d[j] = l - 2;
        }
        sum -= j;
    }
    printf("%ld\n", sum);
    return 0;
}