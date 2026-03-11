#include<stdio.h>
int main(){
  char b;
  scanf("%c", &b);
  if(b == 'A'){
    printf("T");
  }
  if(b == 'C'){
    printf("G");
  }
  if(b == 'G'){
    printf("C");
  }
  if(b == 'T'){
    printf("A");
  }
  return 0;
}