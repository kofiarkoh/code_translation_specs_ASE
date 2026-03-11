#include <stdio.h>
#include <stdlib.h>

int cmpnum(const void * n1, const void * n2) {
	if (*(long *)n1 > *(long *)n2)
	{
		return -1;
	}
	else if (*(long *)n1 < *(long *)n2)
	{
		return 1;
	}
	else
	{
		return 0;
	}
}

int main(void) {

  long n;
  scanf("%ld", &n);
  long a[n];
  for (long i = 0; i < n; i++) {
    scanf("%ld", &a[i]);
  }
  qsort(a, n, sizeof(long), cmpnum);
  long creatures = n;
  long size = a[n-1];
  for (long i = n-1; i > 0; i--) {
    if (a[i-1] > size*2) {
      creatures = i;
    }
    size += a[i-1];
  }
  printf("%ld\n", creatures);

  return 0;
}