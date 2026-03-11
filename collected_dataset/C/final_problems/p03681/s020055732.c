#include<stdio.h>
#include<stdlib.h>
int main(){
  int  i,n,m;
  long long count=1;
  scanf("%d%d",&n,&m);
  if(n==m){
    for(i=1;i<=n;i++){
      count*=(long long)i;
      count%=1000000007;
      if(count==0)
        count=1000000007;
    }
    count*=count;
    count*=2;
    count%=1000000007;
  }
  else if(abs(n-m)==1){
    for(i=n;i>0;i--){
      count*=(long long)i;
      count%=1000000007;
      if(count==0)
        count=1000000007;
    }
    for(i=m;i>0;i--){
      count*=(long long)i;
      count%=1000000007;
      if(count==0)
        count=1000000007;
    }
  }
  else
    count=0;
  printf("%lld\n",count);
  return 0;
}
