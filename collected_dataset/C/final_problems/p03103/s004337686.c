#include <stdio.h>
#include <stdlib.h>

typedef struct drink{
  long price;
  long stock;
}drink;

int compare(const void *a,const void *b){
  drink *A =(drink *)a;
  drink *B =(drink *)b;
  return A->price - B->price;
}

int main(){
  int N,M;
  scanf("%d%d",&N,&M);
  drink* D =(drink*)malloc(sizeof(drink)*N);
  for(int i=0;i<N;i++){
    scanf("%d%d",&D[i].price,&D[i].stock);
  }
  qsort(D,N,sizeof(drink),compare);

  long cost = 0;
  long item = 0;
  for(int j=0;j<N;j++){
    if(item + D[j].stock < M){
      cost += (D[j].price)*(D[j].stock);
      item += D[j].stock;
    }else if(item + D[j].stock >= M){
      cost += (D[j].price)*(M - item);
      item += M;
      j = N+1;
    }
  }
  printf("%ld\n",cost);
  
  return 0;
}