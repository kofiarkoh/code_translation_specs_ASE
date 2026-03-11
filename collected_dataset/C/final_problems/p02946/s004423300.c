#include<stdio.h>
int main()
{
    int k,x,p,i;
    scanf("%d %d",&k,&x);
    p=(k-1)*-1;
    for(i=1;i<k*2;i++)
    {
        printf("%d ",x+p);
        p++;
    }
}
