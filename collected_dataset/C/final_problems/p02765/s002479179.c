#include<stdio.h>

int main(void){
  int n,a;
  scanf("%d",&n);
  scanf("%d",&a);
  if(n<=9){
    printf("%d",a+100*(10-n));    
  }
  else{
    printf("%d",a);
  }
}
