#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void){
  int n;
  char a[100][2];
  int i,j;
  
  //read input
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%s",a[i]);
    if(strcmp(a[i],"Y")==0){
      printf("Four\n");
      return 0;
    }
  }
  printf("Three\n");
  
  return 0;
}

