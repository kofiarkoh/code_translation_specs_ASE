#include <stdio.h>
int main(void)
{
  int n,k;
  int fact;
  scanf("%d %d", &n, &k);
  fact=k-1;
  while(n-->1){
    k*=fact;
  }
  printf("%d\n", k);
}
