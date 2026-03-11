#include<stdio.h>
int main(){
  int N;
  scanf("%d\n",&N);
  int d[N];
  int i;
  int c[101]={};
  for(i=0;i<N;i++){
    scanf("%d",&d[i]);
  }
  for(i=0;i<N;i++){
    c[d[i]]++;
  }
  int count=0;
  for(i=1;i<=100;i++){
    if(c[i]!=0){
      count++;
    }
  }
  printf("%d",count);
  return 0;
}

