#include<stdio.h>
int main(void){
  int n,k,ans=0;
  scanf("%d%d",&n,&k);
  int a[n],i;
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    if(a[i]>=k) ans++;
  }
  printf("%d",ans);
  return 0;
}