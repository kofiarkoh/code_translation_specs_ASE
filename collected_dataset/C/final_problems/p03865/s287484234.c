#include<stdio.h>
int main(){
  int i;
  char s[100010];
  scanf("%s",s);
  for(i=0;s[i];i++);
  if(s[0]==s[i-1])i++;
  printf("%s\n",i%2?"First":"Second");
  return 0;
}

