#include <stdio.h>

int main() {
  int fee_a, fee_b, fee_c, fee_d, cost = 0;
  scanf("%d %d %d %d", &fee_a, &fee_b, &fee_c, &fee_d);
  cost += (fee_a < fee_b)? fee_a : fee_b;
  cost += (fee_c < fee_d)? fee_c : fee_d;
  printf("%d\n", cost);
  return 0;
}
