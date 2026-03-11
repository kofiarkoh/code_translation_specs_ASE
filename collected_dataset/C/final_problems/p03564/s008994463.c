#include <stdio.h>

int main(void) {
  int n, k, ktmp, count = 0, ans = 1, i;
  scanf("%d\n%d", &n, &k);
  ktmp = k;
  if (n == 1) {
    printf("2\n");
    return 0;
  } else {
    // kが2の何乗ぐらいか調べる
    while (ktmp != 0) {
      ktmp = ktmp / 2;
      count++;
    }
    //その回数だけ2倍する
    for (i = 0; i < count; i++) ans = ans * 2;
    //残りの回数kを足す
    ans = ans + (n - count) * k;
    printf("%d\n", ans);
    return 0;
  }
}