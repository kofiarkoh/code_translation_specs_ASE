#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct {
  int B;
  int C;
} st_nums;

int int_sort(const void *a, const void *b) {
  return (*(int*)a - *(int*)b);
}

int nums_sort(const void *a, const void *b) {
  return (((st_nums*)a)->C - ((st_nums*)b)->C);
}

int main(void) {
  int N, M;
  scanf("%d %d", &N, &M);
  int A[100000];
  uint64_t X = 0;
  for(int i = 0; i < N; i++) {
    scanf("%d", &A[i]);
    X += A[i];
  }
  qsort(A, N, sizeof(int), int_sort);

  st_nums nums[100000];
  for(int i = 0; i < M; i++) {
    scanf("%d %d", &nums[i].B, &nums[i].C);
  }
  qsort(nums, M, sizeof(st_nums), nums_sort);

  int dst = 0;
  for(int i = M; i >= 0; i--) {
    for(int j = 0; j < nums[i].B; j++) {
      if(dst >= N)
	break;
      if(A[dst] >= nums[i].C)
	break;
      X += (nums[i].C - A[dst++]);
      //A[dst++] = nums[i].C;
    }
  }

  printf("%ld\n", X);
  return 0;
}
