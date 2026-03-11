#include <stdio.h>
int main(void){
  char b;
  scanf("%s",&b);
  if(b=='A'){
    printf("T");
  }else if(b=='T'){
    printf("A");
  }else if(b=='G'){
    printf("C");
  }else if(b=='C'){
    printf("G");
  }
  return 0;
}