#include <stdio.h>

int main(void){
  char s[200001], s2[200001];
  int count = 0, i=0, j=0;
  scanf("%s", s);
  while(s[i] != '\0'){
    if(s[i] == 'B' && s[i+1] == 'C'){
      s2[j] = 'D';
      j++;
      i++; i++;
      continue;
    }
    s2[j] = s[i];
    i++;
    j++;
  }
  i=0;
  while(s2[i] != '\0'){
    if(s2[i] == 'A' && s2[i+1] == 'D'){
      s2[i] = 'D';
      s2[i+1] = 'A';
      count++;
      i = 0;
      continue;
    }
    i++;
  }
  printf("%d", count);
  return 0;
}