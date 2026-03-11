#include <stdio.h>
#include <string.h>

int main(){
  int i, alp[26] = {};
  char c;
  while((c = getchar()) != EOF) {
    if('a' <= c && c <= 'z') {
      i = (int)(c - 'a');
      alp[i]++;
    }
    if('A' <= c && c <= 'Z') {
      i = (int)(c - 'A');
      alp[i]++;
    }
  }
  c = 'a';
  for(i = 0; i < 26; i++, c++) {
    printf("%c : %d\n", c, alp[i]);
  }
  return 0;
}

