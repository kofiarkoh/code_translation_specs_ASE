#include <stdio.h>
#include <string.h>
#include <ctype.h>
 
int main(void){
 
  char T[1000];
  char W[10];
  int count = 0;
  register int i;
 
  scanf("%s",W);
  for(i=0;i <= strlen(W);i++){
  	if(isupper(W[i])){
      W[i] = tolower(W[i]);
    }
  }
  while(1){
    scanf("%s",T);
    if(strcmp(T,"END_OF_TEXT") == 0){
    	break;
    }
    for(i = 0;i < strlen(T);i++){
      if(isupper(T[i])){
        T[i] = tolower(T[i]);
      }
      if(strcmp(&T[i],W) == 0){
        count++;
      }
    }
  }
  printf("%d\n",count);
  return 0;
}