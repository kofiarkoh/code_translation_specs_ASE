#include <stdio.h>
#include <stdlib.h>

#define IS_SQUARE(d) (((d) & ((d) - 1)) == 0)

int compare_int(const void *a, const void *b)
{
	// 降順 
	return *(int *) b - *(int *) a;
}

int main(int argc, char *argv[])
{
	int n, i, c;
	int j;
	int a[200000];
	
	int x, y;
	int l_x, l_y;
	
	scanf("%d", &n);

	for (i = 0; i < n; i++) {
		scanf("%d", &a[i]);
	}
	
	qsort(a, n, sizeof(int), compare_int);

	c = 0;
	l_x = 0;
	l_y = 0;

	for (i = 0; i < n; i++) {
		x = a[i];
		if (x == 0) {continue;}
		j = i + 1;
		if ((l_x == x)&&(l_y > i)) {
			j = l_y + 1;
		}
		for (; j < n; j++) {
			y = a[j];
			if (y == 0) {continue;}
			if (IS_SQUARE(x + y)) {
				a[j] = 0;
				c++;
				break;
			}
		}
		l_x = x;
		l_y = j;
	}

	printf("%d\n", c);
	
	return 0;
}