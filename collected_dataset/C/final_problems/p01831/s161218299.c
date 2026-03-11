#include<stdio.h>
int main(){
  int d,i,j;
  char s[100010];
  scanf("%d %s",&d,s);
  for(i=0;s[i]    =='<';i++);
  for(j=0;s[d-j-1]=='>';j++);
  //printf("%d %d\n",i,j);
  printf("%d\n",d-(i<j?i:j));
  return 0;
}