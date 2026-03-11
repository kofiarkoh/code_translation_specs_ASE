#include <stdio.h>
#include <stdlib.h>

struct point
{
    int x, y;
};

typedef struct point * P;

int compare(const void *, const void *);
void sort(P [], int);
void pri(P [], int);

int main(void)
{
    int i, n;
    P p, A[100000];

    scanf("%d", &n);
    p = (P) malloc(sizeof(struct point) * n);

    for (i = 0; i < n; i++) {
        A[i] = p + i;
        scanf("%d %d", &(A[i]->x), &(A[i]->y));
    }

    qsort(p, n, sizeof(P), compare);
    pri(A, n);
    return 0;
}

int compare(const void *a, const void *b)
{
    P pa, pb;
    pa = (struct point *) a;
    pb = (P) b;
    return pa->x == pb->x ? pa->y - pb->y : pa->x - pb->x;
}

void sort(P A[], int N)
{
    int i, j;
    P tmp;
    
    for (i = 1; i < N; i++) {
        tmp = A[i];
        for (j = i - 1; j >= 0 && compare(A[j], tmp) > 0; j--)
            A[j + 1] = A[j];
        A[j + 1] = tmp;
    }
}


void pri(P A[], int N)
{
    int i;
    
    for (i = 0; i < N; i++) {
        printf("%d %d\n", A[i]->x, A[i]->y);
    }
}
