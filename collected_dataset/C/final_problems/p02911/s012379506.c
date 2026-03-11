#include <stdio.h>

int main(void){

  long n,k,q;
  scanf("%ld %ld %ld", &n, &k, &q);
  long a[q];
  for (long i = 0; i < q; i++) {
    scanf("%ld", &a[i]);
  }
  long list[n];
  for (long i = 0; i < n; i++) {
    list[i] = 0;
  }
  for (long i = 0; i < q; i++) {
    list[a[i]-1]++;
  }
  for (long i = 0; i < n; i++) {
    list[i] = q-list[i];
    if (list[i] < k) {
      printf("Yes\n");
    } else {
      printf("No\n");
    }
  }

  return 0;
}