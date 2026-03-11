#include <stdio.h>

int main() {
  int i, number_n;
  scanf("%d", &number_n);
  long long int lucas_number[number_n+1];
  lucas_number[0] = 2;
  lucas_number[1] = 1;
  for(i = 2; i <= number_n; i++) {
    lucas_number[i] = lucas_number[i-1] +lucas_number[i-2];
  }
  printf("%lld\n", lucas_number[number_n]);
  return 0;
}
