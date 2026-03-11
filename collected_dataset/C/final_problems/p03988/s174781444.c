#include<stdio.h>

int main(void){
  int N,i,j,k;
  int num[101]={0};

  scanf("%d",&N);
  for(i=0;i<N;i++){
    scanf("%d",&k);
    num[k]++;
  }

  for(i=0;num[i]==0;i++);
  if(num[i]>2){
    puts("Impossible");
    return 0;
  }

  k=i;

  for(i++;num[i]!=0;i++){
    if(num[i]==1){
      puts("Impossible");
      return 0;
    }
  }

  for(j=i+1;j<N;j++){
    if(num[j]!=0){
      puts("Impossible");
      return 0;
    }
  }

  if(i-1>k*2-(num[k]-1)){
    puts("Impossible");
  }else{
    puts("Possible");
  }
  return 0;
}
