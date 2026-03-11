#include <stdio.h>

int main(void) {
  int n, k;
  scanf("%d %d", &n, &k);
  int array[n];

  int i, j, tmp;
  for (i = 0; i < n; i++) {
        scanf("%d", &tmp);
        array[i] = tmp;
  }

  for (i = 0; i < n; i++) {
        for (j = 0; j < n - i - 2; j++) {
          if (array[j] > array[j + 1]) {
                tmp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = tmp;
          }
        }
  }

  int sum;
  sum = 0;
  for (i = 0; i < k; i++) {
        sum += array[n - i - 1];
  }

  printf("%d\n", sum);

  return 0;
}
