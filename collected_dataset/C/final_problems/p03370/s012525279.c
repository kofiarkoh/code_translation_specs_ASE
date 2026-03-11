
#include <stdio.h>

int main()
{
    int n,x,sum=0,min;
    scanf("%d %d",&n,&x);
    int a[n];
    for(int i=0;i<n;i++)
    {
        scanf("%d",&a[i]);
        if(i==0)
            min=a[0];
        sum+=a[i];
        if(min>a[i])
            min=a[i];
        
    }
    printf("%d",(n+((x-sum)/min)));

    return 0;
}
