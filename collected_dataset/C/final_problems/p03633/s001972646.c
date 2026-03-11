#include <stdio.h>

int
main(int argc, char *argv[])
{
  int n; unsigned long long t[100], t_max = 0;
  scanf("%d", &n);
  for(int i = 0; i < n; i++){
    scanf("%llu", &t[i]);
    if(t_max < t[i]) t_max = t[i];
  }

  unsigned long long ans = t_max;
  unsigned long long k;
  for(int i = 0; i < n; i++){
    k = 1;
    while(k * t_max % t[i] != 0) k++;
    ans *= k;
  }
  printf("%llu\n", ans);
  return 0;
}