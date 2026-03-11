#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
typedef long long ll;

int main()
{
    int n;
    ll have=0;
    ll mon=1000;
    ll save=1000;
    scanf("%d",&n);
    int a[n];
    scanf("%d",&a[0]);

    for(int i=1;i<n;i++)
    {
        scanf("%d",&a[i]);

        if(a[i]>a[i-1])
        {
            while(mon>=0)
            {
                have=have+1;
                mon=mon-a[i-1];
            }
            have=have-1;
            mon=mon+a[i-1];
        }
        else if(a[i]<a[i-1])
        {
            while(have>0)
            {
                mon=mon+a[i-1];
                have=have-1;
            }
            save=mon;
        }

    }

    if(have!=0)
    {
        while(have>0)
        {
            mon=mon+a[n-1];
            have=have-1;
        }
        if(mon>save)
        {
            save=mon;
        }
    }

    printf("%lld\n",save);

    
}