#include<stdio.h>
int shisoku(int i);
int main(){
  int n,i,j,ans=0,tmp=0;
  while(1){
    if(scanf("%d",&n) == EOF) break;
    ans = 0;
    for(i=0;i<=n;i++){
      ans += shisoku(i);
    }
    printf("%d\n",ans);
  }
  return 0;
}

int shisoku(int i){
  int n;
  if(i<2) return 0;
  else if(i == 2) return 1;
  if(i % 2 == 0) return 0;
  for(n=3;n<=i/n;n+=2){
    if(i % n == 0) return 0;
  }
  return 1;
}
      