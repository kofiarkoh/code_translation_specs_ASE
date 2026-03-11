#include<stdio.h>
typedef long long int ll;

int main(void){
    ll x;
    scanf("%lld", &x);

    ll q = x/11;
    ll r = x%11;

    ll ans = 0;

    if(r == 0){
        ans = q*2;
    }else if(r <= 6){
        ans = q*2+1;
    }else{
        ans = q*2+2;
    }

    printf("%lld\n", ans);

    return 0;
}