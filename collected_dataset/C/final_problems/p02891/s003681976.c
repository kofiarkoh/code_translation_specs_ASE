#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>
typedef long long ll;
#define rep(i,n) for(ll i=0; i<(ll)(n); i++)
#define max(l,r) ((l)>(r)?l:r)
#define min(l,r) ((l)<(r)?l:r)

int main(){
    //input
    char s[105];
    scanf("%s", s);
    int k;
    scanf("%d", &k);

    //solve
    ll ans = 0;
    int n = strlen(s);
    char t[205];
    rep(i, 2*n) t[i] = s[i%n];
    int allsame = 1;
    rep(i, n-1) if(s[i] != s[i+1]) allsame = 0;
    if(allsame){
        ans = (ll)n * k / 2;
    }else{
        rep(i, n){
            if(t[i] == t[i+1]){
                if(i+1 < n) ans += k;
                else{
                    int tmp = i+1;
                    while(tmp + 1 < 2*n && t[tmp] == t[tmp+1]) tmp++;
                    if((tmp - i) % 2 == 1) ans += k-1;
                    break;
                }
                i++;
            }
        }
    }
    
    //output
    printf("%lld", ans);
}