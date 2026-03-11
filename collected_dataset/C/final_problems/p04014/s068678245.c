#include<stdio.h>
int main(){
  long long n,m,i,j,s;
  scanf("%lld %lld",&n,&m);
  if(n==m){
    printf("%lld\n",n+1);
    return 0;
  }
  for(i=2;i*i<=n;i++){
    s=0;
    for(j=n;j;j/=i)s+=j%i;
    if(s==m){
      printf("%lld\n",i);
      return 0;
    }
  }
  if((i-1)*(i-1)==n)i--;
  for(i--;i;i--){
    j=n/(i+1)+1;
    s=n/j+n%j;
    if(m>s)continue;
    if((s-m)%i==0&&s-m<=n%j&&j+(s-m)/i-1){
      printf("%lld\n",j+(s-m)/i);
      return 0;
    }
  }
  printf("-1\n");
  return 0;
}
