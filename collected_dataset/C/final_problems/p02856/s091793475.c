#include<stdio.h>
int main(){
  long long n,i,j,c=0,a,b,sa=0,ans=0;
  scanf("%lld",&n);
  for(i=0;i<n;i++){//printf("%lld\n",i);
    scanf("%lld %lld",&a,&b);
    sa+=a*b;
    ans+=b;
  }
  printf("%lld\n",ans+sa/10-1);
  return 0;
}
