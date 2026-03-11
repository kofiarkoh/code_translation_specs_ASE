#include <stdio.h>
#include <string.h>

void Print(char S[3]){
  int i;
  if(strlen(S) == 3){
    for(i = strlen(S) - 1;i >= 0;i--) printf("%c", S[i]);
    printf("\n");
  }else printf("%s\n", S);
}

int main(void){
  char S[3];
  scanf("%s", S);
  Print(S);
  return 0;
}
