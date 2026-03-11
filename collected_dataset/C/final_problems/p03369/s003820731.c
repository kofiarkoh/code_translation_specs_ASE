#include <stdio.h>
int main(){
  char s[3];
  int i,t=0;
  scanf("%s",s);
  for(i=0;i<3;i++) {
    if(s[i]-'x')t++;
  }
  printf("%d",700+100*t);
}