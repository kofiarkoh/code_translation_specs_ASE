#include <stdio.h>
int main(void){
  long long i,j,n,m,k,a,b,c,h,w,r=0,l,t,tmp;
  scanf("%lld",&n);
  r=n*(n+1)*(n+2);r/=6;
  for(i=0;i<n-1;i++){
    scanf("%lld%lld",&a,&b);
    if(a>b){tmp=a; a=b; b=tmp;}
    k=a*(n+1-b);
    r-=k;
  }
  printf("%lld\n",r);
  return 0;
}