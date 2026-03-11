#include<stdio.h>

int main()
{
    int a,b,c;
    scanf("%d%d%d",&a,&b,&c);
    if(a!=b&&b!=c&&c!=a)
    printf("3");
    else if(a==b==c)
    printf ("1");
    else printf("2");

    return 0;
}
