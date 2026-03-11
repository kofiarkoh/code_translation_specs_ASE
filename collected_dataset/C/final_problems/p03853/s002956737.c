#include<stdio.h>
int main(){
  int h,w;
  char c[111];
  scanf("%d%d",&h,&w);
  while(h--){
    scanf("%s",c);
    printf("%s\n%s\n",c,c);
  }
}