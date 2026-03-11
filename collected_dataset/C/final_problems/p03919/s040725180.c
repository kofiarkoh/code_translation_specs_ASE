#include <stdio.h>
int main(){
  int h,w;
  char a[6];
  scanf("%d%d",&h,&w);
  for(int i=0;i<h;i++)for(int j=0;j<w;j++){
    scanf("%s",a);
    if(a[2]=='u')printf("%c%d",'A'+j,1+i);
  }
  return 0;
}