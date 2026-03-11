// AOJ 0372: Three Meals
// 2017.11.14 bal4u@uu

#include <stdio.h>
#include <string.h>

long long tbl[3][1440];
int min[3], max[3];

int bitcount64(long long x)
{
	x = ((x & 0xAAAAAAAAAAAAAALL) >>  1) + (x & 0x55555555555555LL);
	x = ((x & 0xCCCCCCCCCCCCCCLL) >>  2) + (x & 0x33333333333333LL);
	x = ((x & 0xF0F0F0F0F0F0F0LL) >>  4) + (x & 0x0F0F0F0F0F0F0FLL);
	x = ((x & 0x00FF00FF00FF00LL) >>  8) + (x & 0xFF00FF00FF00FFLL);
	x = ((x & 0xFF0000FFFF0000LL) >> 16) + (x & 0x00FFFF0000FFFFLL);
	x = ((x & 0xFFFFFF00000000LL) >> 32) + (x & 0x000000FFFFFFFFLL);
	return (int)x;
}

int check(long long a, int j, int *s, int *e)
{
	int i, t1, t2;

	t1 = t2 = 0;
	for (i = min[j]; i <= max[j]; i++) {
		if ((tbl[j][i] & a) == a) {	t2 = i; if (!t1) t1 = i; }
	}
	*s = t1, *e = t2;
	return t1 > 0;
}

int tok(int j, int t, int s1, int e1, int s2, int e2)
{
	if (j == 0) {
		if (t < e1 && e1 < e2) return 1;
	} else if (j == 1) {
		if (s2 < t && t < e1) return 1;
	} else {
		if (s1 < s2 && s2 < t) return 1;
	}
	return 0;
}

int main()
{
	int n, i, j, f, ans;
	long long b;
	
	scanf("%d", &n);
	memset(tbl, 0, sizeof(tbl));
	min[0] = min[1] = min[2] = 1440;
	max[0] = max[1] = max[2] = 0;

	for (b = 1, i = 0; i < n; i++, b <<= 1) for (j = 0; j < 3; j++) {
		int sh, sm, eh, em, t;
		scanf("%d%d%d%d", &sh, &sm, &eh, &em);
		sh = sh*60+sm, eh = eh*60+em;
		if (sh < min[j]) min[j] = sh;
		if (eh > max[j]) max[j] = eh;
		for (t = sh; t <= eh; t++) tbl[j][t] |= b;
	}

	ans = 0;
	for (j = 0; j < 3; j++) {
		int j1 = (j+1)%3, j2 = (j+2)%3;
		for (i = min[j]; i <= max[j]; i++) {
			int s1, e1, s2, e2;
			b = tbl[j][i];
			if (check(b, j1, &s1, &e1) && check(b, j2, &s2, &e2)) {
				if ((f = bitcount64(b)) > ans && tok(j, i, s1, e1, s2, e2)) {
					ans = f;
					if (ans == n) goto done;
				}
			}
		}
	}
done:
	printf("%d\n", ans);
	return 0;
}

/*
5
1:00 2:00   3:30 4:30    6:00 7:00
2:30 3:00   4:00 5:00    5:30 6:30
1:30 2:30   4:30 5:00    6:30 7:00
2:30 3:00   5:00 6:00    6:30 7:00
1:00 2:00   3:00 3:30    4:00 5:00
*/