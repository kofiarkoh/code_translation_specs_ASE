// AOJ 1519: Room of Time and Spirit
// 2017.12.3 bal4u@uu

#include <stdio.h>
#include <string.h>

// <????????????Union Find>
#define MAX 100003

int par[MAX], ws[MAX];

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

int power[100003];

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
	int n, q, a, b, c;

	fgets(p=buf, 40, stdin);
	n = getint(), q = getint();
	memset(par, 0xff, (n+1)*sizeof(int));
	while (q--) {
		fgets(p=buf, 40, stdin);
		if (*p == 'I') {
			p += 3, a = getint(), b = getint(), c = getint();
			if (!power[a] && !power[b]) power[b] = c;
			else if (!power[a]) power[a] = power[b]-c;
			else if (!power[b]) power[b] = power[a]+c;
			unite(a, b, c);
			power[a] += c, power[b] += c;
		} else {
			p += 8, a = getint(), b = getint();
			if (find(a) != find(b)) puts("WARNING");
			else printf("%d\n", power[b] - power[a]);
		}
	}
	return 0;
}