// AOJ 0378: Aerial Photo
// 2018.1.6 bal4u@uu

#include <stdio.h>
#include <string.h>

#define BASE 100
char a[1000][1000]; int aw, ah;
char b[105][105]; int bw, bh;
int z[1000][1000];
char mk[1000][1000];

void Zalgo(int *z, char *s, int n)
{
	int i, k, ll, rr;

	ll = rr = 0;
	for (i = 1; i < n; i++) { 
		if (i > rr) {
			ll = rr = i;
			while (rr < n && (s[rr-ll] == '?' || s[rr] == '?' || s[rr-ll] == s[rr])) rr++;
			z[i] = rr-- - ll;
		} else {
			k = i - ll;
			if (z[k] < rr - i + 1) z[i] = z[k];
			else {
				ll = i;
				while (rr < n && (s[rr-ll] == '?' /*|| s[rr] == '?'*/ || s[rr-ll] == s[rr])) rr++;
				z[i] = rr-- - ll;
			}
		}
	}
}

int main()
{
	int r, c, r2, k, w, h, ans;
	char buf[20];

	fgets(buf, 20, stdin), sscanf(buf, "%d%d%d%d", &aw, &ah, &bw, &bh);
	for (r = 0; r < ah; r++) fgets(a[r]+BASE, 810, stdin);
	for (r = 0; r < bh; r++) fgets(b[r], 105, stdin);
	w = aw+bw, h = ah-bh;

	ans = 0;
	for (r = 0; r <= h; r++) {
		for (r2 = 0; r2 < bh; r2++) {
			memcpy(a[r+r2]+BASE-bw, b[r2], bw);
			Zalgo(z[r+r2], a[r+r2]+BASE-bw, w);
		}

#if 0
printf("r %d\n", r);
for (r2 = 0; r2 < bh; r2++) {
	for (c = 0; c < aw+bw; c++) printf("%d ", z[r+r2][c]);
	printf("\n");
}
printf("\n");
#endif

		k = 0;
		for (c = bw; c <= aw; c++) {
			if (z[r][c] >= bw) mk[r][c] = 1, k++;
		}
		if (k > 0) for (c = bw; c <= aw; c++) {
			if (!mk[r][c]) continue;
			for (r2 = 1; r2 < bh; r2++) if (z[r+r2][c] < bw) {
				mk[r][c] = 0;
				if (--k == 0) goto next;
				break;
			}
		}
		ans += k;
next:;
	}
	printf("%d\n", ans);
	return 0;
}

