#include <math.h>
#include <stdio.h>
 
int main(void) {
  long n, p, i = 1, ans;
  
  scanf("%ld", &n);
  scanf("%ld", &p);
  
  if(n <= 1 || p <= 1) {
    printf("%ld", p);
  }
  
  if(n > 1 && p > 1) {
    while(pow(i, n) <= p) {
      if(p % (long)pow(i, n) == 0) {
        ans = i;
      }
      i++;
    }
    printf("%ld", ans);
  }
  
  return 0;
}