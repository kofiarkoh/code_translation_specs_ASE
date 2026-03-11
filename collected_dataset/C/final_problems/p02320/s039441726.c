#include <stdio.h>
#define WMAX 10000

int dp[WMAX+1], deq[WMAX+1], deqv[WMAX+1];

int main(void)
{
  int i, j, a, n, w, value, weight, limit;
  scanf("%d %d", &n, &w);
  for (i = 0; i < n; i++) {
    scanf("%d %d %d", &value, &weight, &limit);
    for (a = 0; a < weight; a++) {
      int s = 0, t = 0;
      for (j = 0; j * weight + a <= w; j++) {
        int val = dp[j*weight+a] - j * value;
        while (s < t && deqv[t-1] <= val) t--;
        deq[t] = j;
        deqv[t++] = val;
        dp[j*weight+a] = deqv[s] + j * value;
        if (j == deq[s] + limit) s++;
      }
    }
  }
  printf("%d\n", dp[w]);
}
