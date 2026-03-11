#include <stdio.h>
#include <string.h>

int main(){
  char s[10];
  scanf("%s",s);
  int c=0;
  
  for(int i=0;i<4;i++){
    if(s[i]=='2'){
	c++;
    }
  }
	printf("%d", c);
}

