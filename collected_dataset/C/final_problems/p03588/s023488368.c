#define _CRT_SECURE_NO_WARNINGS

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

/* define */
#define Yes(X) puts ( X ? "Yes" : "No")
#define YES(X) puts ( X ? "YES" : "NO")
#define FOR(i, a, b) for ( int i = a; i < b ; i++)
#define REP(i, n) for ( int i = 0; i < n ; i++)
#define max(a, b) (((a) > (b)) ? (a) : (b))
#define min(a, b) (((a) < (b)) ? (a) : (b))
#define max3(a, b, c) ((max((a), (b)) > (c)) ? max((a), (b)) : (c))
#define min3(a, b, c) ((min((a), (b)) < (c)) ? min((a), (b)) : (c))

/* const */
const int MOD = (int)1e9 + 7;

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
int main() {
	int n, a, b, ma = 0, mb;
	scanf("%d", &n);
	REP(i, n) {
		scanf("%d%d", &a, &b);
		if (ma < a) {
			ma = a;
			mb = b;
		}
	}
	printf("%d", ma + mb);
	return 0;
}