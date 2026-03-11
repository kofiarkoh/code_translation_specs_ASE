#include <stdio.h>
#include <stdlib.h>
int main()
{
    char op1='0',op2='0',op3='0';
    int x,a[4];
    while(scanf("%d",&x)==1)
    {
        int i;
        for(i=3;i>=0;--i)
        {
            a[i]=x%10;
            x/=10;
        }
        if(a[0]+a[1]+a[2]+a[3]==7)
        {
            op1=op2=op3='+';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]+a[1]+a[2]-a[3]==7)
        {
            op1=op2='+';
            op3='-';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]+a[1]-a[2]+a[3]==7)
        {
            op1=op3='+';op2='-';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]+a[1]-a[2]-a[3]==7)
        {
            op1='+';op2=op3='-';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]-a[1]+a[2]+a[3]==7)
        {
            op1='-';op2=op3='+';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]-a[1]+a[2]-a[3]==7)
        {
            op1=op3='+';op2='-';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]-a[1]-a[2]+a[3]==7)
        {
            op1=op2='-';op3='+';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else if(a[0]-a[1]-a[2]-a[3]==7)
        {
            op1=op2=op3='-';
            printf("%d%c%d%c%d%c%d=7\n",a[0],op1,a[1],op2,a[2],op3,a[3]);
        }
        else
        {
            printf("no answer!please input another one:");
        }
    }
    return 0;
}
