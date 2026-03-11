#include <stdio.h>
#include <stdlib.h>

int cmpnum(const void * n1, const void * n2) {
	if (*(long *)n1 > *(long *)n2)
	{
		return 1;
	}
	else if (*(long *)n1 < *(long *)n2)
	{
		return -1;
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
  long sum = a[n-1]-a[0];
  long head = 0;
  long tail = n-1;
  long generated_top = a[0];
  long generated_bottom = a[n-1];
  while (1) {
    head++;
    if (head >= tail) {
      break;
    }
    if (labs(a[head]-generated_top) > labs(a[head]-generated_bottom)) {
      sum += labs(a[head]-generated_top);
      generated_top = a[head];
    } else {
      sum += labs(a[head]-generated_bottom);
      generated_bottom = a[head];
    }
    tail--;
    if (head >= tail) {
      break;
    }
    if (labs(a[tail]-generated_top) > labs(a[tail]-generated_bottom)) {
      sum += labs(a[tail]-generated_top);
      generated_top = a[tail];
    } else {
      sum += labs(a[tail]-generated_bottom);
      generated_bottom = a[tail];
    }
  }
  printf("%ld\n", sum);

  return 0;
}