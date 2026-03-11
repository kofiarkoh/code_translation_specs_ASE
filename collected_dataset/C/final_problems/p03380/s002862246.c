#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int cmp(const void *a, const void *b) {
  return *(int*)a > *(int*)b;
}

int main(void) {
  int n;
  int max = -1, maxi = -1, half = -1, halfi = -1;
  int *a;
  scanf("%d", &n);
  a = (int*)calloc(n, sizeof(int));
  for (int i = 0; i < n; i++) {
    scanf("%d", &a[i]);
  }

  for (int i = 0; i < n; i++) {
    if (max < a[i]) {
      maxi = i;
      max = a[i];
    }
  }
  half = max;
  for (int i = 0; i < n; i++) {
    if (abs((double)max/2 - half) >= abs((double)max/2 - a[i]) && maxi != i) {
      halfi = i;
      half = a[i];
    }
  }
  printf("%d %d\n", max, half);

  return 0;
}