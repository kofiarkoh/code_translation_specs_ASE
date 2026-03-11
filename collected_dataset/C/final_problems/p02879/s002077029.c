#include<stdio.h>
int main()
{
    int a,b;
    scanf("%d %d",&a,&b);
    if(a<=9&&b<=9)
    {
        printf("%d\n",a*b);

    }
    else if(a>=10||b>=10)
    {
        printf("-1\n");

    }
    return 0;
}
