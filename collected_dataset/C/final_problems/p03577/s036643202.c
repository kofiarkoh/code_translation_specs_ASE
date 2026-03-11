#include <stdio.h>
int main(){
  char a[50];
  int i,k;
  scanf("%s",a);
  for(i=0;a[i]!='\0';i++);
  for(k=0;k<i-8;k++){
    printf("%c",a[k]);
  }
  printf("\n");


  }
