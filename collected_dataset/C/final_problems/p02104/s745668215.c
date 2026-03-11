// AOJ 3006: Chairs
// 2017.10.4 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 100000
int q[MAX+3], top, end;
int p[MAX+3], a[MAX+3];

int main()
{
	int n, i, k;
	
	scanf("%d", &n);
	top = end = 0;
	for (i = 1; i <= n; i++) scanf("%d", p+i), q[end++] = i;
	memset(a, 0, sizeof(int)*(n+1));
	while (top < end) {
		i = q[top]; if (++top == MAX) top = 0;
		k = p[i];
		if (!a[k]) a[k] = i;
		else {
			if (++p[i] > n) p[i] = 1;
			q[end] = i; if (++end == MAX) end = 0;
		}
	}
	for (i = 1; i <= n; i++) printf("%d\n", a[i]);
	return 0;
}