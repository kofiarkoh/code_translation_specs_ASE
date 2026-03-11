#include <stdio.h>

int main(){
  int X,A,B;
  scanf("%d %d %d",&X,&A,&B);
  if(B<=A) printf("delicious\n");
  else if(B<=X+A) printf("safe\n");
  else printf("dangerous\n");
  return 0;
}