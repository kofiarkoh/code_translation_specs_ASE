#include <stdio.h>
#define maxN 300000
#define repeat(i, N, M) for(int i = N; i < M; i++)

int
main(int argc, char *argv[])
{
  int n; char s[maxN + 1]; int w[maxN] = {}, e[maxN] = {};
  scanf("%d", &n);
  scanf("%s", s);
  
  repeat(i, 1, n){
    w[i] = w[i - 1] + (s[i - 1] == 'W' ? 1 : 0);
    e[i] = e[i - 1] + (s[i - 1] == 'E' ? 1 : 0);
  }

  int arr[maxN]; int ans = maxN;

  repeat(i, 0, n - 1){
    arr[i] = w[i] + e[n - 1] + (s[n - 1] == 'E' ? 1 : 0) - e[i + 1];
    ans = (arr[i] < ans ? arr[i] : ans);
  }
  arr[n - 1] = w[n - 1];
  ans = (arr[n - 1] < ans ? arr[n - 1] : ans);

  printf("%d\n", ans);

  return 0;
}
