#include <stdio.h>

int main(void) {
  int x, y, z, n;
  
  scanf("%d %d %d",&x, &y, &z);
  
  n = x % (z + y);
  if (n < z){
  printf("%d", (x / (z + y) - 1));
  }else{
    printf("%d", (x / (z + y)));
  }
}