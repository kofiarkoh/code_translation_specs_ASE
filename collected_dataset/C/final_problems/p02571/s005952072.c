#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void)
{
  char s[10000],t[10000];
  int max = 0;
  
  scanf("%s%s", s,t);
  
  int ls = strlen(s);
  int lt = strlen(t);
  
  for(int i = 0; i < ls -lt; i++){
    int cnt = 0;
    for(int j = i; j < lt + i -1; j++ ){
      if(strncmp(&s[j],&t[j-i],1)==0){
        cnt++;
      }
    }
    if(max < cnt)
      max = cnt;
  }
  
  printf("%d", lt - max);
  
  return 0;
}