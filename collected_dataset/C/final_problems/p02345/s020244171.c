#include <stdio.h>          // printf(), scanf()

#define MAX_N 100000
#define MAX_INT 0x7fffffff
#define min(a, b) ((a) < (b) ? (a) : (b))

int dat[2 * MAX_N - 1];
int n;

void
init_rmq(int m)
{
	int i;

	n = 1;
	while (n < m)
		n *= 2;

	for (i = 0; i < 2 * n - 1; ++i)
		dat[i] = MAX_INT;
}

void
update(int k, int a)
{
	k += n - 1;
	dat[k] = a;
	while (k > 0)
	{
		k = (k - 1) / 2;
		dat[k] = min(dat[k * 2 + 1], dat[k * 2 + 2]);
	}
}

int
query(int a, int b, int k, int l, int r)
{
	if (r <= a || b <= l)
		return MAX_INT;

	if (a <= l && r <= b)
		return dat[k];

	int vl = query(a, b, k * 2 + 1, l, (l + r) / 2);
	int vr = query(a, b, k * 2 + 2, (l + r) / 2, r);
	return min(vl, vr);
}

int
main(int argc, char** argv)
{
	int q;
	int c, x, y;
	int i;

	scanf("%d %d", &n, &q);
	init_rmq(n);
	for (i = 0; i < q; ++i)
	{
		scanf("%d %d %d", &c, &x, &y);
		if (c == 0)
			update(x, y);
		else
			printf("%d\n", query(x, y + 1, 0, 0, n));
	}

	return 0;
}