#include<stdio.h>
int main()
{
    long int n, a[100005], i, b[100005], j;
    scanf("%ld", &n);
    for(i=0; i<n; i++)
    {
        scanf("%ld", &a[i]);
    }
    for(i=0; i<n; i++)
    {
        j=a[i]-1;
        b[j]=i+1;
    }
    for(j=0; j<n; j++)
    {
        printf("%ld ", b[j]);
    }
    return 0;
}