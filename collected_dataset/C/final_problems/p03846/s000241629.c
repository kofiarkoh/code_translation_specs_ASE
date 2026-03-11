#include <stdio.h>
#include <string.h>
#define MOD 1000000007
int main()
{
    int i,n,a[100000],x,flag=0,sum;
    memset(a,0,sizeof(a));
    scanf("%d",&n);
    for(i=1;n>=i;i++)
    {
        scanf("%d",&x);
        a[x]++;
    }
    if(n%2==0)
    {
        for(i=1;n>i;i++)
        {
            if(i%2==1&&a[i]!=2)
            {
                flag=1;
                break;
            }
            else if(i%2==0&&a[i]!=0)
            {
                flag=1;
                break;
            }
        }
    }
    else
    {
        for(i=0;n>i;i++)
        {
            if(i==0&&a[i]!=1)
            {
                flag=1;
                break;
            }
            else if(i!=0&&i%2==0&&a[i]!=2)
            {
                flag=1;
                break;
            }
            else if(i%2==1&&a[i]!=0)
            {
                flag=1;
                break;
            }
        }
    }
    if(flag==1)
    {
        printf("0\n");
    }
    else
    {
        sum=1;
        for(i=1;n/2>=i;i++)
        {
            sum=(sum*2)%MOD;
        }
        printf("%d\n",sum);
    }
    return 0;
}