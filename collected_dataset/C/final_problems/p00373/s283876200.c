// AOJ 0378: Aerial Photo
// 2017.11.23 bal4u@uu

#include <stdio.h>

typedef long long ll;
ll a[802][802]; int aw, ah;
ll b[102][102]; int bw, bh;
ll tr[128];

int main()
{
	int i, r, c, r2, c2, w, h, ans;
	ll k;
	char m[802];

	for (k = 1, i = 0; i < 10; i++, k <<= 1) tr[i+'0'] = k;
	for (i = 0; i < 26; i++, k <<= 1) tr[i+'A'] = k;
	for (i = 0; i < 26; i++, k <<= 1) tr[i+'a'] = k;
	tr['?'] = k-1;

	scanf("%d%d%d%d", &aw, &ah, &bw, &bh);
	for (r = 0; r < ah; r++) {
		scanf("%s", m);
		for (c = 0; c < aw; c++) a[r][c] = tr[m[c]];
	}
	for (r = 0; r < bh; r++) {
		scanf("%s", m);
		for (c = 0; c < bw; c++) b[r][c] = tr[m[c]];
	}
	ans = 0; w = aw-bw, h = ah-bh;
	for (r = 0; r <= h; r++) for (c = 0; c <= w; c++) {
		for (r2 = 0; r2 < bh; r2++) for (c2 = 0; c2 < bw; c2++) {
			if (!(b[r2][c2] & a[r+r2][c+c2])) goto next;
		}
		ans++;
next:;
	}
	printf("%d\n", ans);
	return 0;
}