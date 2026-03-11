#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define _max(_x, _y) ((_x)>(_y)?(_x):(_y))
#define _min(_x, _y) ((_x)>(_y)?(_y):(_x))

int main(void)
{
  int N, H, index;
  int ans = 0;
  int max_height = 0;
  scanf("%d", &N);
  for (index = 0; index < N; ++index) {
    scanf("%d", &H);
    if (H >= max_height) {
      ans++;
      max_height = H;
    }
  }
  printf("%d\n", ans);
  
  return 0;
}