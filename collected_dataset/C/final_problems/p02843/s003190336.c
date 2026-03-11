#include<stdio.h>

int main(){
  int x;
  int k;
  int i  =0;
  int flag = 0;
  scanf("%d",&x);
  k = x/100;
  for(i=0;i<=k;i++){
    if(105*i >= x && 100*i <= x){
      flag = 1;
      break;
    }
  }
    
  printf("%d",flag);
  
  return 0;
}