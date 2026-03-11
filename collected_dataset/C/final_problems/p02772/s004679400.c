#include<stdio.h>
int main()
{
    int a,i,x,m,n;
    m=0,n=0;
    scanf("%d",&a);
    for(i=1;i<=a;i++)
    {
        scanf("%d",&x);
        if(x%2==0)
        {
            m++;
            if(x%3==0 || x%5==0)
            {
                n++;
            }
        }
    }
    if(m==n)
    {
        printf("APPROVED\n");
    }
    else
    {
        printf("DENIED\n");
    }
    return 0;
}
