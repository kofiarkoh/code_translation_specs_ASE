#include <stdio.h>
int main() {
  int N;
  scanf("%d", &N);
  int L[10];
  int i,j; /* インデックス */
  for (i = 0; i < N; ++i) {
    scanf("%d",&L[i]);
  }

  int temp;
  for (i = 0; i < N ; ++i) {
    for	(j = i + 1; j < N; ++j) {
      if (L[i] < L[j]) {
        temp = L[i];
        L[i] = L[j];
        L[j] = temp;
      }
    }
  }

  int sum = 0;
  for (i = 1; i < N; ++i) {
    sum += L[i];
  }

  if (L[0] >= sum) {
    printf("No\n");
  } else {
    printf("Yes\n");
  }

  return 0;
}