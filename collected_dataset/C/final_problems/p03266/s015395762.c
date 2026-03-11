#include <stdio.h>
#include <math.h>

int main(void) {

  long n,k;
  scanf("%ld %ld", &n, &k);
  long mod_0 = n/k;
  long mod_half = mod_0;
  if ((n%k)*2 >= k) {
    mod_half++;
  }
  if (k%2 != 0) {
    mod_half = 0;
  }
  long answer = pow(mod_0,3)+pow(mod_half,3);
  printf("%ld\n", answer);

  return 0;
}