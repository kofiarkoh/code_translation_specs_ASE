#include<stdio.h>
int main(){
  int N, M, X, Y;
  scanf("%d %d %d %d", &N, &M, &X, &Y);
  
  int xn[101];
  int xm[101];
  for(int i = 0; i < N; i++){
    scanf("%d",&xn[i]);
    if(X < xn[i]){
      X = xn[i];
    }
  }
  
  for(int i = 0; i < M; i++){
    scanf("%d",&xm[i]);
    if(Y > xm[i]){
      Y = xm[i];
    }
  }
  if(X >= Y){
    printf("War");
  }else{
    printf("No War");
  }
}
