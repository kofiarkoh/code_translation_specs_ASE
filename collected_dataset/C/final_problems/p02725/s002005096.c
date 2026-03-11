#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include<time.h>
void printarray(int arr[], int n);
int partition(int arr[], int i, int j, int pivot)
{
    int l, r, k, t;
    l = i;
    r = j;
    while (l <= r)
    {
        while (l <= r && arr[l] <= pivot)
            {l++;}
        while (l <= r && arr[r] >= pivot)
            {r--;}
        if (l <= r)
        {
            t = arr[l];
            arr[l] = arr[r];
            arr[r] = t;
            l++;r--;
        }
    }
    k = l - 1;
    arr[i - 1] = arr[k];
    arr[k] = pivot;
    // printf("Pivot index : %d\n",k);
    // printarray(arr,n);
    return k;
}
void RQS(int arr[], int i, int j)
{
    int t, p, k;
    if (i < j)
    {   srand(time(0));
        p = (rand()%(j-i+1)) + i;
        // p = 1;
        t = arr[i];
        arr[i] = arr[p];
        arr[p] = t;
        k = partition(arr,i+1,j, arr[i]);
        RQS(arr, i,k - 1);
        RQS(arr, k + 1, j);
        // printarray(arr,n);
    }
}
void printarray(int arr[], int n)
{
    int i;
    for (i = 0; i < n; i++)
    {
        printf("%d ", arr[i]);
    }
    printf("\n");
}
int main()
{
    int ans,n,i,k;
    scanf("%d%d",&k,&n);
    int arr[n];
    for(i=0;i<n;i++)
    {
        scanf("%d",&arr[i]);
    }
    ans = k;
    int  j;
    i = 0;
    j = n-1;
    RQS(arr,i, j);
    for(i=0;i<n-1;i++)
    {
        if(ans > k - (arr[i+1]-arr[i]))
        {
            ans = k - (arr[i+1]-arr[i]);
        }
    }
    if(arr[n-1]-arr[0]<ans)
    {
        ans = arr[n-1]-arr[0];
    }
    printf("%d",ans);
    return 0;
}
