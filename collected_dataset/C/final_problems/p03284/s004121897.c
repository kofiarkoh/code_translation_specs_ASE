#include <stdio.h>

int main(void){
  int n, k;
  scanf("%d %d", &n, &k);
  if(n < k){
    printf("1\n");
  }else{
    int a = n / k;
    int b = a * k;
    printf("%d\n", n - b);
  }
  return 0;
}