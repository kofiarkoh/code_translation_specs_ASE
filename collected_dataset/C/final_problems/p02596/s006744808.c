#include <stdio.h>

int main(void){
    int k,l;
    int mod,ans;
    scanf("%d",&k);

    if (k%7==0)
    {
        l=9*k/7;
    } else
    {
        l=9*k;
    }

    if (l%2==0 || l%5==0)
    {
        printf("-1\n");
        return 0;
    }

    mod=1;
    ans=0;
    for (int i=0; i<l; i++)
    {
        mod*=10;
        mod=mod%l;
        if (mod==1)
        {
            ans=i+1;
            break;
        }
    }
    printf("%d\n",ans);
    return 0;
}
