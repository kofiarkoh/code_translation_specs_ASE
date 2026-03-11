#include<stdio.h>
int main(){
  long long n,m,i,j,ans=1;
  scanf("%lld %lld",&n,&m);
  if(n==1){
    printf("%lld\n",m);
    return 0;
  }
  for(i=2;i*i<=m;i++){
    for(j=0;m&&m%i==0;j++)m/=i;
    for(;j>=n;j/=n)ans*=i;
  }
  printf("%lld\n",ans);
  return 0;
}
