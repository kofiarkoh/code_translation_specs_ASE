#include<stdio.h>
int main(){
  long long n,m;
  scanf("%lld %lld",&n,&m);
  if(m==1)printf("%lld\n",(n*(n-1))/2);
  else{
    n-=m-2;
    printf("%lld\n",(n*(n-1))/2+m-3);
  }
  return 0;
}