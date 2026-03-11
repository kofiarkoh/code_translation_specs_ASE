#include<stdio.h>
#include<stdlib.h>
#include<math.h>
int main(void){
  int N,K,i,j,temp,min;
  int h[10000]={};
  double H[10000]={};
  min=1000000000;
  if(scanf("%d%d",&N,&K)>2){
    exit(1);
  }
  for(i=0;i<N;i++){
    if(scanf("%lf",&H[i])>1){
    exit(1);
  }
  }
  for(i=0;i<N;i++){
    h[i]=H[i];
  }
  for(i=0; i<N; i++){
    for(j=N-1; j>i; j--){
      if(h[j] < h[j-1]){
        temp = h[j];
        h[j] = h[j-1];
        h[j-1] = temp;
      }
    }
  }
  for(i=0;i<N;i++){
    //printf("%d",h[i]);
  }
  for(i=0;i<N-K+1;i++){
    if(min>h[i+K-1]-h[i]){
    min=h[i+K-1]-h[i];
  }
  }
  printf("%d",min);
  return 0;
}

