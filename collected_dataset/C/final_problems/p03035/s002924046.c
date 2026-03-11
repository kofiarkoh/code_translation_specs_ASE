#include<stdio.h>
int main(){
  int A,B;
  scanf("%d %d",&A,&B);
  if(A<=5) printf("0\n");
  else if(6<=A&&A<=12) printf("%d",B/2);
  else printf("%d",B);
  return 0;
}
                         