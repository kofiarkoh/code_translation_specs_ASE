#include <stdio.h>
#include <stdlib.h>

int main()
{
    int l,r,d,i,count=0;
    scanf("%d%d%d",&l,&r,&d);
    for(i=l;i<=r;i++)
    {
        if(i%d==0)
        {
            count++;
        }
    }
    printf("%d\n",count);
    return 0;
}
