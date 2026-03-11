#include <stdio.h>
#include <stdlib.h>
int f(int *a,int *b)
{
    int i,s=0;
    for(i=0;a+i!=b;i++)
        s+=a[i];
    return s;
}
int main()
{
    int n,i,m;
    scanf("%d",&n);
    int a[n],b[n];
    for(i=0;i<n;i++)
        scanf("%d",&a[i]);
    for(i=0;i<n;i++)
        scanf("%d",&b[i]);
    for(i=0;i<n;i++)
    {
        if(f(a,a+i+1)+f(b+i,b+n)>m)m=f(a,a+i+1)+f(b+i,b+n);
    }
    printf("%d\n",m);
    return 0;
}
