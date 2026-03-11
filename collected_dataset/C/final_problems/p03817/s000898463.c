#include<stdio.h>

int main(){
  long x = 0;
  long times = 0;
  scanf("%ld",&x);
  times = (x / 11)*2;
  if(x % 11 != 0 && x % 11 <= 6){
    times += 1;
  }else if(x % 11 >= 7){
    times += 2;
  }
  
  printf("%ld",times);
  return 0;
}
    
  
  