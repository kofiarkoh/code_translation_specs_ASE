#include <stdio.h>

int main(){
  int n;
  long long ans=2,T=1,A=1;
  scanf("%d",&n);
  
  int t[n+1],a[n+1];
  for(int i=1;i<=n;i++) scanf("%d %d",&t[i],&a[i]);
  
  for(int i=1;i<=n;i++){
    for(int j=1;j>=0;j++){
      if(j*t[i]>=T && j*a[i]>=A){
        T=j*t[i];
        A=j*a[i];
        ans=(a[i]+t[i])*j;
        break;
      }
    }
  }
  
  printf("%lld",ans);
  return 0;
}
