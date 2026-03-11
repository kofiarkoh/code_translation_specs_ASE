#include <stdio.h>
typedef long long ll;

ll min(ll a,ll b){return a<b?a:b;}

ll DigitSum(ll b,ll n){
    ll ans=0;
    while(n!=0){
        ans+=n%b;
        n/=b;
    }
    return ans;
}

int main(void){
    ll n,s,ans=-1;
    scanf("%lld%lld",&n,&s);
    if (s>n){
        ans=-1;
    } else if (n==s){
        ans=n+1;
    } else {
        for (ll i=2; i*i<=n; i++){
            if (DigitSum(i,n)==s){
                ans=i;
                break;
            }
        }
        if (ans==-1){
            for (ll i=1; i*i<=n; i++){
                ll b=(n-s)/i+1;
                if ((n-s)%i==0 && DigitSum(b,n)==s){
                    ans=b;
                    break;
                }
            }
        }
    }
    printf("%lld\n",ans);
	return 0 ;
}