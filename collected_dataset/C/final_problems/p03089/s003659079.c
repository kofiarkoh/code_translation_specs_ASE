#include <stdio.h>

int main(void) {
  int n,b[101]={},ans[101]={},N,t_N,flag;
  scanf("%d",&N);
  t_N=N;
  for(int i=1;i<=t_N;i++){scanf("%d",&b[i]);}
  while(t_N!=0){
    flag=1;
    for(int i=t_N;i>=1;i--){
      if(b[i]==i){
        for(int j=i;j<=t_N-1;j++){b[j]=b[j+1];}
        ans[t_N]=i;
        b[t_N]=0;
        t_N--;
        flag=0;
        break;
      }
    }
    if(flag){
      printf("-1");
      return 0;
    }
  }
  for(int i=1;i<=N;i++){printf("%d\n",ans[i]);}
  return 0;
}