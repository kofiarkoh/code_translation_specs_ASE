#include <stdio.h>

int main(void){
  int i,x,d,n;
  
  scanf("%d %d",&d,&n);
  
  for(x=1,i=0;i<d;x*=100,i++);
  
  printf("%d\n",x*n);
  
  return 0;
}