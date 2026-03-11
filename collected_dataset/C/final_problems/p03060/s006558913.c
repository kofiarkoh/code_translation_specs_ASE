#include<stdio.h>

int main(void){

  int N;
  scanf("%d",&N);
  int i;
 
  int V[N];
  int C[N];
  int ans=0;
  
  for(i=0;i<N;i++){
    scanf("%d",&V[i]);
  }
  for(i=0;i<N;i++){
    scanf("%d",&C[i]);
  }
 
  for(i=0;i<N;i++){
    if(V[i]-C[i]>0) ans = ans + V[i]-C[i];
    else{}
  }

  printf("%d\n",ans);

  return 0;
}


