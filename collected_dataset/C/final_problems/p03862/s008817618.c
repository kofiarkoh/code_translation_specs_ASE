#include<stdio.h>
int main()
{
    int n,x;
    while(scanf("%d %d",&n,&x)!=EOF)
    {
        int a[100002];
        for(int i=0;i<n;i++)
            scanf("%d",&a[i]);
        long long int sum=0;
        if(a[0]>x)
        {
            sum+=a[0]-x;
            a[0]=x;
        }

        for(int i=1;i<n;i++)
        {
            if(a[i]+a[i-1]>x)
            {
                sum+=a[i]+a[i-1]-x;
                a[i]=x-a[i-1];
            }
        }
        printf("%lld\n",sum);
    }
    return 0;
}
