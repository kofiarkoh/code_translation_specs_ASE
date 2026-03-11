#include <stdio.h>

#define MOD_BY 1000000007

int add(int a, int b) {
	return a + b - MOD_BY * (a + b >= MOD_BY);
}

int sub(int a, int b) {
	return b == 0 ? a : add(a, MOD_BY - b);
}

int mul(int a, int b) {
	int r = 0;
	while (b) {
		if (b & 1) r = add(r, a);
		a = add(a, a);
		b >>= 1;
	}
	return r;
}

int power(int a, int b) {
	int r = 1;
	while (b) {
		if (b & 1) r = mul(r, a);
		a = mul(a, a);
		b >>= 1;
	}
	return r;
}

int inv(int a) {
	return power(a, MOD_BY - 2);
}

int waru(int a, int b) {
	return b == 0 ? 0 : mul(a, inv(b));
}

int cache[3333333];

int mizuhasu[333333];

int asumisu[333333];

int main(void) {
	int r1, c1, r2, c2;
	int i;
	int hoge, fuga;
	int kotae;
	cache[0] = 1;
	for (i = 1; i < 3333333; i++) {
		cache[i] = mul(cache[i - 1], i);
	}
	if (scanf("%d%d%d%d", &r1, &c1, &r2, &c2) != 4) return 1;
	for (i = c1; i <= c2; i++) {
		mizuhasu[i] = waru(waru(cache[r1 + i], cache[r1]), cache[i]);
	}
	hoge = r2 - r1;
	fuga = c2 - c1;
	for (i = 0; i <= fuga + 1; i++) {
		asumisu[i] = waru(waru(cache[hoge + i], cache[hoge]), cache[i]);
	}
	kotae = 0;
	for (i = c1; i <= c2; i++) {
		kotae = add(kotae, mul(mizuhasu[i], asumisu[c2 - i + 1]));
	}
	for (i = r1 + 1; i <= r2; i++) {
		int a = waru(waru(cache[i + c1 - 1], cache[i]), cache[c1 - 1]);
		int b = waru(waru(cache[(r2 - i + 1) + (fuga + 1)], cache[r2 - i + 1]), cache[fuga + 1]);
		kotae = add(kotae, mul(a, sub(b, 1)));
	}
	printf("%d\n", kotae);
	return 0;
}

/*

対象範囲の一番下がa, b, cのとき、和 (別にx, yも後で足す)

y a aab aaabbc
x a ab  abc
  a b   c

下から上に行った時、合計でa, b, cをそれぞれ何個足すか

  10 4  1
  6  3  1
  3  2  1
  1  1  1

これは右下から左上に向かう組み合わせになっている！

そして、xを足す数は、aを足す数-1
yを足す数も、上に行く数を減らせば同様に求まる

*/
