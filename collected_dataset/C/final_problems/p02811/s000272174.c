#include <stdio.h>

int main(void) {
  /* code */
  int k,x;
  scanf("%d %d", &k, &x);
  if(500*k >= x){
    printf("%s\n","Yes" );
  }else{
    printf("%s\n","No" );
  }
  return 0;
}