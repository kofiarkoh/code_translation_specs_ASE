#include <stdio.h>

int main(void) {
  int r,g,b;

  scanf("%d %d %d",&r,&g,&b);

  if((2*g+b)%4==0){
    printf("YES\n");
  }
  else{
    printf("NO\n");
  }
  return 0;
}
