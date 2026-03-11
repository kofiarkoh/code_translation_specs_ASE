#include <stdio.h>
#include <stdlib.h>

#define SENTINEL 1000000000

int count = 0;
void merge(int[], int, int, int);
void mergeSort(int A[], int left, int right)
{
    int i, mid;
    if (left + 1 < right)
    {
        mid = (left + right) / 2;
        mergeSort(A, left, mid);
        mergeSort(A, mid, right);
        merge(A, left, mid, right);
    }
}
void merge(int A[], int left, int mid, int right)
{
    int n1, n2, i, j, k;
    int *L, *R;
    n1 = mid - left;
    n2 = right - mid;
    L = (int *)malloc(sizeof(int) * (n1 + 1));
    R = (int *)malloc(sizeof(int) * (n2 + 1));
    for (i = 0; i <= n1 - 1; i++)
    {
        L[i] = A[left + i];
    }
    for (j = 0; j <= n2 - 1; j++)
    {
        R[j] = A[mid + j];
    }
    L[n1] = SENTINEL;
    R[n2] = SENTINEL;
    i = 0;
    j = 0;
    for (k = left; k <= right - 1; k++)
    {
        if (L[i] <= R[j])
        {
            A[k] = L[i];
            i++;
            count++;
        }
        else
        {
            A[k] = R[j];
            j++;
            count++;
        }
    }
    free(L);
    free(R);
}

int main(void)
{
    int N, a[100000];
    int i, cnt = 0, temp = 0, sum = 0;

    scanf("%d", &N);

    for (i = 0; i < N; i++)
    {
        scanf("%d", &a[i]);
    }
    mergeSort(a, 0, N);
    /*
for(i=0;i<N;i++){
    printf("%d",a[i]);
    if(i<N-1)printf(" ");
  }
*/
    temp = a[0];
    for (i = 0; i < N; i++)
    {
        if (temp == a[i])
        {
            cnt++;
        }
        else
        {
            //printf("cnt:%d\ntemp:%d\n",cnt,temp);
            if (cnt >= temp)
            {
                sum += (cnt - temp);
            }
            else
            {
                sum += cnt;
            }
            //printf("%d\n",sum);
            temp = a[i];
            cnt = 1;
        }
    }
    
    if (cnt >= temp)
    {
        sum += (cnt - temp);
    }
    else
    {
        sum += cnt;
    }
    printf("%d", sum);

    return (0);
}