#include <stdio.h>
#define SENTINEL 1000000000
int a[200010],count = 0;
void merge(int left,int mid,int right)
{
    int n1 = mid - left, n2 = right - mid;
    int l[n1 + 10],r[n2 + 10];
    int i,j,k;
    for(i = 0;i < n1;i++)
        l[i] = a[left + i];
    for(i = 0;i < n2;i++)
        r[i] = a[mid + i];
    i = j = 0;
    l[n1] = r[n2] = SENTINEL;
    for(k = left;k < right;k++)
    {
        if(l[i] < r[j])
            a[k] = l[i++];
        else
        {
            count++;
            a[k] = r[j++];
        }
    }
}
void mergesort(int left,int right)
{
    if(left + 1 < right)
    {
        int mid = (left + right) / 2;
        mergesort(left,mid);
        mergesort(mid,right);
        merge(left,mid,right);
    }
}
int main()
{
    int n,i;
    scanf("%d",&n);
    for(i = 0;i < n;i++)
        scanf("%d",&a[i]);
    mergesort(0,n);
    printf("%d\n",count-1);
    return 0;
}