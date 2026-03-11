#include <stdio.h>
typedef long long ll;
#define rep(i,n) for(int i=0; i<(int)(n); i++)

int main(){
    int n, p; scanf("%d%d",&n,&p);
    int a[101010], cnt[2] = {0};
    rep(i,n) scanf("%d",a+i);
    ll ans = 0;

    rep(i,n) cnt[(a[i])%2]++;
    ans = (1LL << (cnt[1]-1)) * (1LL << cnt[0]);
    printf("%lld",ans);
}