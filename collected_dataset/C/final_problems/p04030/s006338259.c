#include<stdio.h>
int main(){
  char s[11];
  char a[11];
  int i;
  int c=0;
  for(i=0;i<11;i++)a[i]='\0';
  scanf("%s",s);
  for(i=0;i<10;i++){if(s[i]!='B'){a[c]=s[i];c++;}else{c=c==0?0:c-1;a[c]='\0';}}
  printf("%s",a);
}