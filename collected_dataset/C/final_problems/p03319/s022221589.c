#include<stdio.h>

int main(){
  int i = 0;
  int n = 0;
  int k = 0;
  scanf("%d %d",&n,&k);
  for(i=0;i<n;i++){
    if(n <= k + (k-1)*i){
      break;
    }
  }
  
  printf("%d",i+1);
  return 0;
}