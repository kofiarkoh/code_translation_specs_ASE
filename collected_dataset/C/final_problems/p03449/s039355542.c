#include <stdio.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))

int
main(int argc, char *argv[])
{
  int n, a[2][100];
  scanf("%d", &n);
  for(int i = 0; i < n; i++) scanf("%d", &a[0][i]);
  for(int i = 0; i < n; i++) scanf("%d", &a[1][i]);

  int ca; int max = 0;

for(int i = 0; i < n; i++){
ca = 0;
for(int j = 0; j <= i; j++) ca += a[0][j];
for(int j = i; j < n; j++) ca += a[1][j];
max = Max(max, ca);
}

printf("%d\n", max);

return 0;
}
