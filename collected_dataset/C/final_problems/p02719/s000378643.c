#include<stdio.h>
typedef long long ll;
ll min(ll x,ll y){
    if(x<y) return x;
    else return y;
}
int main(){
    long long N,K;
    scanf("%lld %lld",&N,&K);
    printf("%lld",min(N%K,K-N%K));        

}
