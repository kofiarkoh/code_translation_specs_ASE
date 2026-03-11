#include<stdio.h>
#define ll long long

void swap(ll *a,ll *b){
  ll tmp = *a;
  *a=*b;
  *b=tmp;
}

int main(){
  ll i,j,n,c[200004],q=0,z=1e9+7;
  scanf("%lld",&n);
  for(i=0;i<n;i++){
    scanf("%lld",&c[i]);
  }
  if(n==1) q=c[0]*2%z;
  else{
    for(i=0;i<n-1;i++){
      for(j=i;j<n;j++){
        if(c[i]>c[j]){
          swap(&c[i],&c[j]);
        }
      }
    }
    for(i=0; i<n; i++){q+=(n+1-i)*c[i], q%=z;}
    for(i=0; i<2*n-2; i++){q*=2; q %= z;}
  }
  printf("%lld",q);
}