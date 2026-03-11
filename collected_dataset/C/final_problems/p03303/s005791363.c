#include <stdio.h>
int main()
{
  char s[1000]={0};
  int i,w;
  scanf("%s",s);
  scanf("%d",&w);
  for(i=0;i<1000;i++){
    if(s[i]==0) break;
    if(i%w==0) printf("%c",s[i]);
  }
  return 0;
}