#include <stdio.h>

int main(void){
  int n,m,a,b,flag=0;
  int fl[200001]={0};
  scanf("%d %d",&n,&m);
  for(int i=0;i<m;i++){
    scanf("%d %d",&a,&b);
    if(a==1){
      fl[b]=fl[b]+1;
    }
    if(b==n){
      fl[a]=fl[a]+1;
    }
  }
  for(int i=1;i<=n;i++){
    if(fl[i]==2){
      printf("POSSIBLE\n");
      flag=1;
      break;
    }
  }
  if(flag==0)printf("IMPOSSIBLE\n");
  return 0;
}
