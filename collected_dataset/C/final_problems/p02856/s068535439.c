#include<stdio.h>
int main(){
  long long n,i,j,c=0,a,b,sa=0,sb=0;
  scanf("%lld",&n);
  for(i=0;i<n;i++){//printf("%lld\n",i);
    scanf("%lld %lld",&a,&b);
    sa+=a;
    sb+=b;
  }
  for(i=0;sa>10;i++){
    c=0;
    for(j=sa;j>10;j/=10){
      sb++;
      c+=j%10;
    }
    sa=c;
  }
  printf("%lld\n",sb-1);
  return 0;
}
