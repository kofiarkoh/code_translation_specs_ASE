#include <stdio.h>
int main(void){
  int i;
  char  s[1024];
  char t[8]={'k','s','t','n','h','m','y','r'};
  scanf("%s",s);
  for(i=0;s[i];i=i+2){
    if(s[i]=='0'&&s[i+1]=='U')
      printf("nn");
    else{
      if(s[i]=='0')
	putchar('w');
      else if('2'<=s[i]&&s[i]<='9')
	putchar(t[s[i]-'0'-2]);
      switch(s[i+1]){
      case 'T':putchar('a');break;
      case 'L':putchar('i');break;
      case 'U':putchar('u');break;
      case 'R':putchar('e');break;
      case 'D':putchar('o');
      }
    }
  }
  puts("");
  return 0;
}