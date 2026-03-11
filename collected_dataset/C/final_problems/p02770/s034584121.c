#include<stdio.h>
#include<stdlib.h>
#include<string.h>

#define ll long long

int main(){
  int k,q;
  scanf("%d%d",&k,&q);

  ll d[5010];
  for(int i=0;i<k;i++){
    scanf("%lld",&d[i]);}

  for(int i=0;i<q;i++){
    ll n,x,m;
    scanf("%lld %lld %lld",&n,&x,&m);
    ll an=x%m,tmp;
    ll zeros=0;
    ll amari = (n-1)%k;
    for(int j=0;j<k;j++){
      tmp = d[j]%m;
      if(amari>j){
	if(tmp==0) zeros+=(n-1)/k+1;
	an += ((n-1)/k+1)*tmp;}
      else{
	if(tmp==0) zeros+=(n-1)/k;
	an+=((n-1)/k)*tmp;}}
    //printf("%lld %lld %lld\n",zeros,an,n-1);
    ll ans = n-1-zeros-an/m;
    printf("%lld\n",ans);}
  return 0;
}
