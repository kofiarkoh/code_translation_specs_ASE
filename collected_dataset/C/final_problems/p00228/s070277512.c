#include <stdio.h>

int main(void) {
  int a[10][7] = {{1,1,1,1,1,1,0},
		  {0,1,1,0,0,0,0},
		  {1,1,0,1,1,0,1},
		  {1,1,1,1,0,0,1},
		  {0,1,1,0,0,1,1},
		  {1,0,1,1,0,1,1},
		  {1,0,1,1,1,1,1},
		  {1,1,1,0,0,1,0},
		  {1,1,1,1,1,1,1},
		  {1,1,1,1,0,1,1},};
  int n, m, current[7], bit[7], i, j;

  while (1) {
    scanf("%d", &n);
    if (n == -1) break;

    for (i = 0; i < 7; i++)
      current[i] = 0;

    for (i = 0; i < n; i++) {
      scanf("%d", &m);
      for (j = 0; j < 7; j++) {
	if (a[m][j] == current[j])
	  bit[j] = 0;
	else
	  bit[j] = 1;
      }
      for (j = 0; j < 7; j++)
	current[j] = a[m][j];
      for (j = 6; j >= 0; j--)
	printf("%d", bit[j]);
      putchar('\n');
    }
  }
  return 0;
}