#include<stdio.h>
int main()
{
    int n,i,a[10000];
    scanf("%d",&n);
    for(i=1; i<100; i++)
    {
       a[i]=360*i;
    }
    for(i=1; i<100; i++)
    {
        if(a[i]%n==0)
            printf("%d",a[i]/n);{
        break;
            }
    }
    return 0;
}
