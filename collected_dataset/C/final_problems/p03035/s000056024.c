#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void){
  int A,B;
  char str[100],*p;
  
  fgets(str,sizeof(str),stdin);
  p=strtok(str," \n");
  A=atoi(p);
  p=strtok(NULL," \n");
  B=atoi(p);
  
  if(A>12){
    printf("%d\n",B);
  }
  if(A<13 && A>5){
    printf("%d\n",B/2);
  }
  if(A<6){
    printf("0\n");
  }
  
  return 0;
}