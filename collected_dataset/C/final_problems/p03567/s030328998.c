#include <stdio.h>
int main(void){
  char s[5];
  scanf("%s",s);
  for(int i=0;s[i]!='\0';i++){
      if(s[i]=='A'&&s[i+1]=='C') {printf("Yes");return 0;}
  }
  printf("No");
  return 0;
}