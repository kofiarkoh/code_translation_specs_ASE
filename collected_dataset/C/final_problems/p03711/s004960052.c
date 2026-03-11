#include <stdio.h>
#include <stdlib.h>

int main()
{
    int a[7]={1,3,5,7,8,10,12};
    int b[4]={4,6,9,11};
    int a1,b1,t,n,i,j;

    while(~scanf("%d %d",&a1,&b1))
    {
        if(a1==2&&b1==2)
            printf("Yes\n");
        else if(a1==2&&b1!=2||a1!=2&&b1==2)
            printf("No\n");
        else
        {
            t=n=0;
        for(i=0;i<7;i++)
        {
            if(a1==a[i])
                t++;
                if(b1==a[i])
                t++;
        }
        for(j=0;j<4;j++)
        {
            if(a1==b[j])
               n++;
            if(b1==b[j])
                n++;
        }
        if(t==2||n==2)
            printf("Yes\n");
        else
            printf("No\n");
        }

    }

    return 0;
}
