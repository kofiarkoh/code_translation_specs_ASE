#define _CRT_SECURE_NO_WARNINGS

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>


/* define */
#define Yes(X) puts ( (X) ? "Yes" : "No")
#define YES(X) puts ( (X) ? "YES" : "NO")
#define FOR(i, a, b) for ( int i = (a); i < (b) ; i++)
#define REP(i, n) for ( int i = 0; i < (n) ; i++)
#define max(a, b) (((a) > (b)) ? (a) : (b))
#define min(a, b) (((a) < (b)) ? (a) : (b))
#define max3(a, b, c) ((max((a), (b)) > (c)) ? max((a), (b)) : (c))
#define min3(a, b, c) ((min((a), (b)) < (c)) ? min((a), (b)) : (c))

/* const */
const int MOD = 1000000007;

/* alias */
typedef long long ll;
typedef unsigned  u;

/* function */
int cmp(const void *a, const void *b) {
	return *(int*)a - *(int*)b;
}
int cmp2(const void *a, const void *b) {
	return *(int*)b - *(int*)a;
}
int gcd(int a, int b) {
	if (b != 0)return gcd(b, a % b);
	return a;
}
ll Lgcd(ll a, ll b) {
	if (b != 0) return Lgcd(b, a % b);
	return a;
}
int lcm(int a, int b) {
	return a / gcd(a, b) * b;
}
ll Llcm(ll a, ll b) {
	return a / Lgcd(a, b) * b;
}

/* main */

int n, a[200000], s[200000] = {}, c = 0;

int main() {
	ll k;
	scanf("%d%lld", &n, &k);
	REP(i, n)scanf("%d", &a[i]);
	REP(l, k) {
		for (int i = 0; i < n; i++) {
			int f = 1;
			REP(j, c) {
				if (a[i] == s[j]) {
					f = 0;
					c = j;
					break;
				}
			}
			if (f) {
				s[c] = a[i];
				c++;
			}
		}

		if (c == 0) {
			k %= (l + 1);
			l = -1;
		}
	}

	REP(i, c)printf("%d ", s[i]);
	return 0;
}