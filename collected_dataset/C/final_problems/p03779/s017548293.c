#include<stdio.h>
#include<stdlib.h>

int main(){
  long x = 0;
  long i = 0;
  
  scanf("%ld",&x);
  for(i=0;i<1000000000+1;i++){
    if(x <= i*(i+1)/2){
      break;
    }
  }
  printf("%ld",i);
  return 0;
}
    