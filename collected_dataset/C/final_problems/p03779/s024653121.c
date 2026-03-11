#include <stdio.h>
#include <stdlib.h>

int main()
{
    int i,x,s=0;
    scanf("%d",&x);
    for(i=1;s<x;i++)
        s+=i;
    printf("%d\n",--i);
    return 0;
}
