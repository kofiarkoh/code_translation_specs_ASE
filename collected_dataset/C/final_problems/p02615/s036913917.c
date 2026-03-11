#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SIZE_OF_ARRAY(str) (sizeof(str) / sizeof(int))

int compareInt(const void* a, const void* b) {
  int aNum = *(int*)a;
  int bNum = *(int*)b;

  return bNum - aNum;
  //降順でソート
}

int main() {
  int N;
  scanf("%d", &N);
  int str[N];
  for (int i = 0; i < N; i++) {
    scanf("%d", &str[i]);
  }

  qsort(str, SIZE_OF_ARRAY(str), sizeof(int), compareInt);
  int sum = 0;
  for (int k = 0; k < N - 1; k++) {
    sum += str[k];
  }
  printf("%d\n", sum);
  return 0;
}