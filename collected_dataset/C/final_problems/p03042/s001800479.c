#include<stdio.h>
int main()
{
    int a,b,m,n,i,j,k,l;
    scanf("%d",&n);

    a=n/100;
    b=n%100;

      if((a>=1 && a<=12) && (b>=1 && b<=12))
        printf("AMBIGUOUS\n");
        else if((a>=1 && a<=12) && (b>12))
        printf("MMYY\n");
    else if((b<=12 && b>=1) && a>12)
        printf("YYMM\n");

    else
        printf("NA\n");

    return 0;
}
