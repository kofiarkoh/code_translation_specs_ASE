#include<stdio.h>
int main (void){
  int n,cnt,i1;
  int ans=0;
  scanf("%d",&n);
  for(int i=1;i<n+1;i++){
    cnt=0;
    i1=i;
    while(i1>0){
      i1=i1/10;
      cnt++;
    }
    if(cnt%2==1){
      ans++;
    }
  }
  printf("%d",ans);
  return 0;
}
