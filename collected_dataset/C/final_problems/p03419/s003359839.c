#include <stdio.h>

int main(void){
  long long n,m,tmp;

  scanf("%lld%lld",&n,&m);
  if(n>m){
    tmp=n;
    n=m;
    m=tmp;

  }

  if(n==1&&m==1){
    printf("1");
    return 0;
  }
   if(n==1&&m!=1) printf("%lld",m-2);
   else printf("%lld",(m-2)*(n-2));

  return 0;
}
