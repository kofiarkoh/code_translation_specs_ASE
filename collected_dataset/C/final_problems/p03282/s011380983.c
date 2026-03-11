#include<stdio.h>
#include<string.h>

int main(void) {
  char S[101];
  int K,i,len;
  
  scanf("%s",S);
  scanf("%d",&K);
  
  i = 0;
  len = strlen(S);
  
  while((i+1 < K)&&(S[i] == '1')&&(i+1 < len)) 
  	i++;  
  printf("%c\n",S[i]);
  return(0);
}
  
