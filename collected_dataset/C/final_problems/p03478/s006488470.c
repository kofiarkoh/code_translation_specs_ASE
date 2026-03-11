#include <stdio.h>

int main(void){

  int n,a,b;
  scanf("%d %d %d", &n, &a, &b);
  long sum = 0;
  int digit_sum;
  int number;
  for (int i = 1; i <= n; i++) {
    number = i;
    digit_sum = 0;
    for (int j = 0; j < 5; j++) {
      digit_sum += number%10;
      number /= 10;
    }
    if (digit_sum >= a && digit_sum <= b) {
      sum += i;
    }
  }
  printf("%ld\n", sum);

  return 0;
}