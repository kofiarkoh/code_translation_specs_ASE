#include<stdio.h>
long long MAX(long long a,long long b){return a<b?b:a;}
int main(){
  long long n,a,b,d[100010],l,r,m,i,s;
  scanf("%lld %lld %lld",&n,&a,&b);
  for(i=l=0;i<n;i++)scanf("%lld",&d[i]);
  for(r=1e9+10;l-r;){//printf("%lld %lld\n",l,r);
    m=(l+r)/2;
    for(i=s=0;i<n;i++)s+=MAX(0,((d[i]-m*b)-1)/(a-b)+1);
    if(s>m)l=m+1;
    else   r=m;
  }
  printf("%lld\n",l);
  return 0;
}
