#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))
#define ABSS(a, b) ((a) > (b) ? (a) - (b) : (b) - (a))

typedef struct {
  int X;
  int Y;
} st_nums;

int main(void) {
  int N;
  scanf("%d", &N);

  st_nums nums[50];
  for(int i = 0; i < N; i++) {
    scanf("%d %d", &nums[i].X, &nums[i].Y);
  }
  int minCost = N;
  for(int i = 0; i < N-1; i++) {
    for(int j = i+1; j < N; j++) {
      int p = nums[i].X - nums[j].X;
      int q = nums[i].Y - nums[j].Y;
      int cost = N;
      for(int m = 0; m < N-1; m++) {
	for(int n = m+1; n < N; n++) {
	  if((nums[m].X-p) == (nums[n].X) &&
	     (nums[m].Y-q) == (nums[n].Y))
	    cost--;
	}
      }
      minCost = MIN(minCost, cost);
    }
  }
  printf("%d\n", minCost);
  return 0;
}
