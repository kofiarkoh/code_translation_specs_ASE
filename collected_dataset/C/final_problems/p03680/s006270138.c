#include <stdio.h>
int main(void)
{
  int n, a[100002], i, count=0, in=1, tmp;
  scanf("%d", &n);
  for(i = 0; i < n; i++) scanf("%d", &a[i]);

  while(1){
    count++;
    if(a[in-1] == 2) break;
    if(a[in-1] == 0){
      printf("-1\n");
      return 0;
    }
    tmp = in-1;
    in = a[in-1];
    a[tmp] = 0;
  }
  printf("%d\n", count);
  return 0;
}
