#include <stdio.h>

int compare(int C1[2], int C2[2])
{
    if (C1[0] < C2[0]) return 1;
    else if (C1[0] == C2[0]) {
        if (C1[1] < C2[1]) return 1;
    }
    return 0;
}

void insertionSort(int A[][2], int n, int g)
{
    int i;
    int C[2];
    for (i = g; i < n; i++) {
        int x, y, j;
        C[0] = A[i][0];
        C[1] = A[i][1];
        j = i - g;
        while (j >= 0 && compare(C, A[j])) {
            A[j + g][0] = A[j][0];
            A[j + g][1] = A[j][1];
            j -= g;
        }
        A[j + g][0] = C[0];
        A[j + g][1] = C[1];
    }
}

void shellSort(int A[][2], int n)
{
    int i, m = 0;
    int G[10] = {262913, 65921, 16577, 4193, 1073, 281, 77, 23, 8, 1};

    for (i = 0; i < 10; i++) {
        if (G[i] <= n) {
            m++;
            insertionSort(A, n, G[i]);
        }
    }
}

int main(void)
{
    int n;
    scanf("%d", &n);

    int A[100000][2];
    int i;
    for (i = 0; i < n; i++) scanf("%d %d", &A[i][0], &A[i][1]);

    shellSort(A, n);

    for (i = 0; i < n; i++) printf("%d %d\n", A[i][0], A[i][1]);

    return 0;
}
