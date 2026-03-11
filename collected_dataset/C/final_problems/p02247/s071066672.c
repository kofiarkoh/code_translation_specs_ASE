#include <stdio.h>
#include <stdlib.h>
#include <string.h>
  
int main(int argc, char **argv){
  char T[1001];
  char P[1001];
  int i, j, count, len;
  
  scanf("%s", T); 
  scanf("%s", P); 
    
  len=strlen(P);
  for(i=0;T[i]!='\0';i++) {
    if (T[i]==P[0]) {
      count=1;
      for(j=1;(T[i+j]==P[j])&&(P[j]!='\0');j++){
        count++;
      }   
      if(count==len){
        printf("%d\n", i); 
      }   
    }   
  }
  return 0;
}