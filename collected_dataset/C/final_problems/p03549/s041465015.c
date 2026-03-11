#include <stdio.h>
int main(void)
{
    long long int n,m;
    scanf("%lld %lld",&n,&m);
    long long int ans=0;
    ans+=1800*m+100*n;
    for(int p=1;p<m+1;p++)ans*=2;
    printf("%lld",ans);
}
