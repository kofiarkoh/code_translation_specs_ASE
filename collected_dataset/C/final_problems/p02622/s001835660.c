#include <stdio.h>
#include <string.h>
#define NUMBER 20000
int main()
{
  char s[NUMBER],t[NUMBER];
  scanf("%s", s);
  scanf("%s", t);
  int i,cnt=0,len=0;
  while(s[len])
  len++;
  for(i=0;i<len;i++){
    if(s[i] == t[i])
    continue;
    else
    cnt++;
  }
printf("%d", cnt);
  return 0;
}
