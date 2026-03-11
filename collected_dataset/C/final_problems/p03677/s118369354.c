#include <stdio.h>

int main()
{
	int i, n, m, a, b, diff;
	long long sum = 0, add[100001][2] = {};
	scanf("%d %d", &n, &m);
	scanf("%d", &a);
	a--;
	for (i = 2; i <= n; i++) {
		scanf("%d", &b);
		b--;
		diff = (b - a + m) % m;
		sum += diff;
		if (diff >= 2) {
			a = (a + 2) % m;
			b = (b + 1) % m;
			add[a][0]++;
			add[a][1]++;
			add[b][0]--;
			add[b][1] -= diff;
			if (b < a) {
				add[0][0]++;
				add[0][1] += m - a + 1;
			}
		}
		a = (b + m - 1) % m;
	}
	
	long long tmp[2] = {}, max = 0;
	for (i = 0; i < m; i++) {
		tmp[0] += add[i][0];
		tmp[1] += add[i][1];
		if (tmp[1] > max) max = tmp[1];
		tmp[1] += tmp[0];
	}
	printf("%lld\n", sum - max);
	fflush(stdout);
	return 0;
}