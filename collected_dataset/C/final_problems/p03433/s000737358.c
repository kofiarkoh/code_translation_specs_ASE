#include<stdio.h>
int main()
{
    int n,a,m;
    scanf("%d%d",&n,&a);
    m = (n%500);
    if(m<=a || m==0)
    {
        printf("Yes");
    }
    else
        printf("No");
    return 0;
}
