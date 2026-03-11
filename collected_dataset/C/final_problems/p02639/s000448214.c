#include "stdio.h"

int main(void)
{
    int x[5];

    int i;
    for(i=0;i<=4;i++)
    {
        scanf("%d",&x[i]);
    }

    for(i=0;i<=4;i++)
    {
        if(x[i]==0)
        {
            printf("%d",i+1);
        }
    }
    return 0;
}