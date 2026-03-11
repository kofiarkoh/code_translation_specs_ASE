// AOJ 1330: Never Wait for Weights
// 2017.12.3 bal4u@uu

#include <stdio.h>
#include <string.h>

// <????????????Union Find>
#define MAX 100003

int par[MAX], ws[MAX];

#if 0
void initWeightedUnionFind()
{
	int i;
	for (i = 0; i < n; i++) par[i] = -1;
}
#endif

int find(int x)
{
	int p;
	if (par[x] < 0) return x;
    p = find(par[x]), ws[x] += ws[par[x]];
    return par[x] = p;
}
    
int weight(int x)
{
    find(x); 
    return ws[x];
}
    
int unite(int x, int y, int w)  // x <-(w)- y (x + w = y)
{
	int tmp;

    w += weight(x), w -= weight(y);
    x = find(x), y = find(y);
    if (x == y) return 0;
    if (par[y] < par[x]) // ???????????????????????????????????????????????????.
        tmp = x, x = y, y = tmp, w = -w;
    
    par[x] += par[y], par[y] = x;
    ws[y] = w;
    return 1;
}
    
int same(int x, int y)
{
	return find(x) == find(y);
}

// *ans = x - y. ?????????????????? return 0.
int diff(int *ans, int x, int y)
{
    if (find(x) != find(y)) return 0;
	*ans = ws[x] - ws[y];
	return 1;
}
// </????????????Union Find>

char buf[40], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
	return n;
}

int main()
{
	int n, m, q, a, b;

	while (fgets(p=buf, 40, stdin) && *p != '0') {
		n = getint(), m = getint();
		memset(par, 0xff, (n+1)*sizeof(int));
		memset(ws, 0, (n+1)*sizeof(int));
		while (m--) {
			fgets(p=buf, 40, stdin);
			q = *p, p += 2, a = getint(), b = getint();
			if (q == '!') unite(b, a, getint());
			else {
				if (find(a) != find(b)) puts("UNKNOWN");
				else printf("%d\n", ws[a] - ws[b]);
			}
		}
	}
	return 0;
}