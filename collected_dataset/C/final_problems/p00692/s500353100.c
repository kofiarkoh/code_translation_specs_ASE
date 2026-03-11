// Aizu 1110: Patience
// 2017.9.14 bal4u@uu

#include <stdio.h>

int pair [20][5] = {
	{1,4,5}, {2,4,5,6}, {3,5,6,7}, {6,7}, {5,8,9},
	{6,8,9,10}, {7,9,10,11}, {10,11}, {9,12,13}, {10,12,13,14},
	{11,13,14,15}, {14,15}, {13,16,17}, {14,16,17,18}, {15,17,18,19},
	{18,19}, {17}, {18}, {19} };

int ans;

int combi(long long cd, int n)
{
	int i, a, b, p, q;
	long long nc, b1, b2, b3;

	if (n < ans) ans = n;
	if (n == 0) return 1;
	for (p = 0; p < n; p++) {
		a = (cd >> (p*3)) & 7;
		for (i = 0; q = pair[p][i]; i++) {
			if ((b = (cd >> (q*3)) & 7) != a) continue;
			b1 =  cd             & ((1ULL << (p*3))-1);
			b2 = (cd >> (p+1)*3) & ((1ULL << (q-p-1)*3)-1);
			b3 = (cd >> (q+1)*3) & ((1ULL << (n-q-1)*3)-1);
			nc = b1 | (b2 << (p*3)) | (b3 << (q-1)*3);
			if (combi(nc, n-2)) return 1;
		}
	}
	return 0;
}

int main()
{
	int n, i, k[20];
	long long card;

	scanf("%d", &n);
	while (n-- > 0) {
		for (i = 0; i < 20; i++) scanf("%d", k+i);
		for (card = 0, i = 19; i >= 0; i--) card = (card << 3) | k[i];
		ans = 20, combi(card, 20);
		printf("%d\n", ans);
    }
	return 0;
}