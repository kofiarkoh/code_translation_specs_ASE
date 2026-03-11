#include<stdio.h>
int a[100005];
int main()
{
    int n,i,k,sum;
    scanf("%d %d",&n,&k);
    for(i=0;i<n;i++)
    {
        scanf("%d",&a[i]);
    }
    if(n<=k)
    {
        printf("1\n");
    }
    else
    {
       sum=(n-k)/(k-1)+1+1;
        printf("%d\n",sum);
    }
    return 0;
}
