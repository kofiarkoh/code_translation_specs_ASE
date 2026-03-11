// AOJ 1576: Community Integration
// 2017.10.23 bal4u@uu

#include <stdio.h>

#define ABS(a) ((a)>=0?(a):-(a))

#define MAX 1001

/* UNION-FIND library */
int p[MAX], rank[MAX];
void make_set(int x) { p[x] = x, rank[x] = 0; }
void link(int x, int y) {
	if (rank[x] > rank[y]) p[y] = x;
	else { p[x] = y; if (rank[x] == rank[y]) rank[y] = rank[y] + 1;	}
}
int find_set(int x) { if (x != p[x]) p[x] = find_set(p[x]);	return p[x]; }
void union_set(int x, int y) { link(find_set(x), find_set(y)); }

int f[MAX];

int main() 
{
	int n, m, i, a, b;

	scanf("%d%d", &n, &m);
	for (i = 1; i <= n; i++) make_set(i);
	while (m-- > 0) {
		scanf("%d%d", &a, &b);
		union_set(a, b);
	}
	for (i = 1; i <= n; i++) f[find_set(i)]++;
	for (a = b = 0, i = 1; i <= n; i++) {
		if (f[i] == 1) a++; else if (f[i] > 1) b++;
	}
	printf("%d\n", ABS(a-b));
	return 0; 
}