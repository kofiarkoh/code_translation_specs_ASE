#include <stdio.h>
//#include <string.h>

int main(void){

  //char s[11];
  int a,b,c = 0;
  scanf("%d %d %d", &a, &b, &c);


  if((a-b)<c) printf("%d", c-a+b);
  else printf("0");
  
  return 0;
}

  

