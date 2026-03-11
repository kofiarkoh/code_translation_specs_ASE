#include <stdio.h>
#include <string.h>
int main(void){
  char s[100000];
  int i;
  scanf("%s",s);
  for(i=0;i<strlen(s);i+=2){
    printf("%c",s[i]);
  }
  return 0;
}