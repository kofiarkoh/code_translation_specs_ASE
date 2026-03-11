#include<stdio.h>
#include<stdlib.h>
int a[100005];
int asc(const void* x, const void* y)
{
	if (a[*(int*)x] > a[*(int*)y])return 1;
	if (a[*(int*)x] < a[*(int*)y])return -1;
	return 0;
}
int main()
{
	int n;
	scanf("%d", &n);
	int i;
	for (i = 0; i < n; i++)
		scanf("%d", &a[i]);
	int c[100005];
	for (i = 0; i < n; i++)
		c[i] = i;
	qsort(c, n, sizeof(int), asc);
	int ans = 0;
	for (i = 0; i < n; i++)
		if (i % 2 != c[i] % 2)
			ans++;
	printf("%d\n", ans / 2);
	return 0;
}