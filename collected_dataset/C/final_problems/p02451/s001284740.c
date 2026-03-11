#include <stdio.h>
#include <stdbool.h>

bool binarySearch(int [], int, int);

int main(void)
{
    int i, n, q, k, A[100000];

    scanf("%d", &n);
    for (i = 0; i < n; i++)
        scanf("%d", A + i);
    
    scanf("%d", &q);
    while (q--) {
        scanf("%d", &k);
        printf("%d\n", binarySearch(A, n, k));
    }
    
    return 0;
}

bool binarySearch(int A[], int N, int k)
{
    int low, mid, high;
    low = 0;
    high = N;
    while (low < high) {
        mid = (low + high) / 2;
        if (A[mid] == k)
            return true;
        else if (A[mid] < k)
            low = mid + 1;
        else
            high = mid;
    }
    return false;
}
