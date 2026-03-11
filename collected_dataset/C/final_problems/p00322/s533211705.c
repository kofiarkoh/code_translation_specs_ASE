// Aizu Vol-3 0327: Alphametic
// 2017.8.25

#include <stdio.h>

int d[10], f[10];
int b[10] = { 0, 1, 10, 1, 100, 10, 1, 100, 10, 1 };
int cnt;

void combi(int i, int n, int s)
{
	int j, t;

	if (i == 10) { if (n == s) cnt++; return; }
	if (d[i] >= 0) {
		t = d[i] * b[i];
		if (i >= 7)	combi(i + 1, n, s + t);
		else        combi(i + 1, n + t, s);
	} else {
		for (j = 1; j <= 9; j++) {
			if (f[j]) continue;
			f[j] = 1;
			t = j * b[i];
			if (i >= 7)	combi(i + 1, n, s + t);
			else        combi(i + 1, n + t, s);
			f[j] = 0;
		}
	}
}

int main()
{
	int i;

	for (i = 1; i <= 9; i++) f[i] = 0;
	for (i = 1; i <= 9; i++) {
		scanf("%d", d + i);
		if (d[i] >= 0) f[d[i]] = 1;
	}
	cnt = 0;
	combi(1, 0, 0);
	printf("%d\n", cnt);
	return 0;
}