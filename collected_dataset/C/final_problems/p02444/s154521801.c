#include <stdio.h>

void pri(int [], int);
void reverse(int [], int, int);
void rotate(int [], int, int, int);

int main(void)
{
    int i, n, q, b, m, e, A[1000];

    scanf("%d", &n);
    for (i = 0; i < n; i++)
        scanf("%d", A + i);

    scanf("%d", &q);
    for (i = 0; i < q; i++) {
        scanf("%d %d %d", &b, &m, &e);
        rotate(A, b, m, e);
    }

    pri(A, n);
    return 0;
}

void rotate(int A[], int b, int m, int e)
{
    reverse(A, b, m);
    reverse(A, m, e);
    reverse(A, b, e);
}

void reverse(int A[], int b, int e)
{
    int tmp;

    e--;
    while (b < e) {
        tmp = A[b];
        A[b] = A[e];
        A[e] = tmp;
        b++;
        e--;
    }
}

void pri(int A[], int N)
{
    int i;
    
    for (i = 0; i < N; i++) {
        if (i > 0) putchar(' ');
        printf("%d", A[i]);
    }
    putchar('\n');
}
