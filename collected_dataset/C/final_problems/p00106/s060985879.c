// Aizu Vol-1 0106: Discounts of Buckwheat 
// 2017.8.12 bal4u@uu

#include <stdio.h>

int tbl[60];
int a[3] = { 2, 3, 5 };
int s[3] = { 380, 550, 850 };
int c[3] = { 10, 12, 15 };
int f[3] = { 1520, 1870, 2244 };

void combi(int i, int v, int cost)
{
	int j, cc;

	if (i == 3) {
		if (v <= 50 && tbl[v] == 0 || tbl[v] > cost) tbl[v] = cost;
		return;
	}
	for (j = 0; v+j <= 50; j += a[i]) {
		cc = j / c[i] * f[i] + (j % c[i]) / a[i] * s[i];
		combi(i + 1, v + j, cost + cc);
	}
}

int main()
{
	int n;

	combi(0, 0, 0);
	while (1) {
		scanf("%d", &n); if (!n) break;
		printf("%d\n", tbl[n / 100]);
	}
	return 0;
}