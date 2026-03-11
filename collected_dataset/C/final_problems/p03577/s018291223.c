#include<stdio.h>

void run(void){
  char s[51];
  scanf("%s",s);
  int n=0;
  while(s[n]!='\0') n++;
  s[n-8]='\0';
  printf("%s\n",s);
}

int main(void){
  run();
  return 0;
}