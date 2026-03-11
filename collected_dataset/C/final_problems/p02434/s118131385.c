#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    int n;
    scanf("%d", &n);
    int *A;
    A = malloc(sizeof(int) * n * 500000);
    int I[1000] = {0};

    int q;
    scanf("%d", &q);
    int op, t, x, i, j;

    while (q--) {
        scanf("%d", &op);
        scanf("%d", &t);
        if (op == 0) {
            scanf("%d", &x);
            A[500000 * t + I[t]] = x;
            I[t] += 1;
        }
        else if (op == 1) {
            if (I[t] == 0) printf("\n");
            else {
                j = 500000 * t;
                printf("%d", A[j]);
                for (i = 1; i < I[t]; i++) printf(" %d", A[j + i]);
                printf("\n");
            }
        }
        else I[t] = 0;
    }

    return 0;
}
