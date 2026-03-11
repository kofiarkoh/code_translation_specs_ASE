#include <stdio.h>
#include <string.h>

int main(){
  char A[4][10]={"Sunny","Cloudy","Rainy"};
  char S[10];
  scanf("%s",S);
  int i=0;
  while(strcmp(S,A[i])!=0) i++;
  printf("%s\n",A[(i+1)%3]);
  return 0;
}