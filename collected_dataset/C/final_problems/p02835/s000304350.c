#include<stdio.h>
int main()
{
    int A1,A2,A3,s;
    scanf("%d %d %d",&A1,&A2,&A3);
    s=A1+A2+A3;
    if(s>=22)
    {
        printf("bust\n");
    }
    else
    {
        printf("win\n");
    }

    return 0;
}
