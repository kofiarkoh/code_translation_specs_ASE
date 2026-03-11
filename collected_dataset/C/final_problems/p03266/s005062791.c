#include <stdio.h>

int main(void) 
{
  int n, k;
  scanf("%d %d", &n, &k);
  if( k % 2 != 0) {
    printf("%ld\n", (n/k) * (n/k) * (n/k) );
  }
  else {
    int mod = k/2, i;
    for(i = 0; k*i+mod <= n; ++i) { ; } 
    printf("%ld\n", (n/k) * (n/k) * (n/k) + i*i*i);
  }
  return 0;
}