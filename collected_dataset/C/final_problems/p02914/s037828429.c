#include <stdio.h>

void swap (long long int *a, long long int *b) {
  long long int tmp = *a;
  *a = *b;
  *b = tmp;
}

int main(){
  int n;
  scanf("%d", &n);
  long long int a[n];
  long long int sum = 0;
  for(int i = 0; i < n; i++){
    scanf("%lld", &a[i]);
    sum = sum ^ a[i];
  }
  //printf("%lld\n", sum);
  long long int mask = ~sum;
  for (int i = 0; i < n; i++) {
    a[i] = a[i] & mask;
  }
  int rank = 0;
  long long int ans = 0;
  for (int i = 59; i >= 0; i--) {
    if (mask & (1uLL << i)) {
      int p = -1;
      for (int j = rank; j < n; j++) {
        if (a[j] & (1uLL << i)) {
          p = j;
          if (j != rank) {
            swap(&a[j], &a[rank]);
          }
          rank++;
          break;
        }
      }
      if (p > 0) {
        for (int j = 0; j < n; j++) {
          //printf("%llx %llx\n", a[j], (1uLL << i));
          if ((a[j] & (1uLL << i)) && (j != rank - 1)) {
            a[j] ^= a[rank - 1];
          }
        }
      }
    }
  }
  for(int i = 0; i < n; i++){
    ans = ans ^ a[i];
  }
  ans = (ans << 1) + sum;
  printf("%lld\n", ans);
  return 0;
}
