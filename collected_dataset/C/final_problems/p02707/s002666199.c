#include<stdio.h>
int main()
{
    int a,i,b[200000],c[200000]={0};
    scanf("%d",&a);
    for(i=0;i<a-1;i++)
    {
        scanf("%d",&b[i]);

    }
    for(i=0;i<a;i++)
    {
        c[b[i]]++;
    }
    for(i=1;i<=a;i++)
    {
        printf("%d\n",c[i]);
    }
}