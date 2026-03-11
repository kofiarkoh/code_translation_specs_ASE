#include <stdio.h>

int main(void){
  int n;
  int x;
  
  scanf("%d", &n);
  x=n%10;
  
  if(x==3){
    printf("bon");
  }else if(x==0||x==1||x==6||x==8){
    printf("pon");
  }else{
    printf("hon");
  }
  
  return 0;
}