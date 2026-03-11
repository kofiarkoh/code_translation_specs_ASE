#include <stdio.h>

int next_permutation(int *A, int size)
{
    if (size == 1) return 0;

    int i, flag = 0;
    for (i = size - 2; i >= 0; i--) {
        if (A[i] < A[i+1]) {
            flag = 1;
            break;
        }
    }
    
    if (flag == 0) return 0;

    int j, tmp;
    for (j = size - 1; j > i; j--) {
        if (A[j] > A[i]) {
            tmp = A[j];
            A[j] = A[i];
            A[i] = tmp;
            break;
        }
    }
    int mid = (size - i - 1) / 2;
    for (j = 1; j <= mid; j++) {
        tmp = A[i+j];
        A[i+j] = A[size-j];
        A[size-j] = tmp;
    }
    return 1;
}

void print_array(int *A, int size)
{
    int i;
    printf("%d", A[0]);
    for (i = 1; i < size; i++) printf(" %d", A[i]);
    putchar('\n');
}

int main(void)
{
    int n;
    scanf("%d", &n);

    int A[9];
    int i;
    for (i = 0; i < n; i++) A[i] = i + 1;

    do {
        print_array(A, n);
    } while (next_permutation(A, n));

    return 0;
}
