#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

int main() {
  int n,i,j;
  int x = 0;
  long a[200000];
  int count = 0;
  scanf("%d", &n);
  for(i = 0; i < n; i++) {
    if(i != n-1) scanf("%ld ", a+i);
    else scanf("%ld", a+i);
  }
  for(i = 0; i < n; i++) {
    for(j = 0; j < n; j++) {
      if(i != j && a[i] % a[j] != 0) {
        count++;
      }
    }
    if(count == n-1) {
      x++;
    }
    count = 0;
  }
  printf("%d\n",x);
  return 0;
}
