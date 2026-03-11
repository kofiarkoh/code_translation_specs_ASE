#include <stdio.h>

int main()
{
    int i,j,n,m,tmp;
    scanf("%d %d",&n,&m);
    if(n > m){tmp = n;m = n;m = tmp;}
    if(n >= 2 && m >= 2)
    {
        printf("%d",(n - 2) * (m - 2));
    }
    else if(n == 1 && m >= 2)
    {
        printf("%d",m - 2);
    }
    else
    {
        printf("0");
    }
    
}