#include<stdio.h>
#define max(a,b) ((a)>(b)?(a):(b))
typedef long long int ll;
#define MOD 1000000007
int main() {
    int n;
    scanf("%d",&n);
    long a[200000];
    long sum=0;
    ll ans=0;
  	for (int i=0;i<n;i++) {
        scanf("%ld",&a[i]);
        ans=(ans+(ll)sum*a[i])%MOD;
        sum+=a[i]%MOD;
    }
    printf("%lld",ans);
    return 0;
}