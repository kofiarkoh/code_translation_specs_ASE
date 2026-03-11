#include<stdio.h>
int main()
{
    int n,s=0,a,i;
    scanf("%d",&n);
    for(i=0;i<n;i++)
    {
        scanf("%d",&a);
        s+=a;
    }
    s-=n;
    printf("%d\n",s);
    return 0;
}
