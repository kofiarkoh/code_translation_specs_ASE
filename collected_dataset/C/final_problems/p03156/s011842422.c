#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

/* define*/
#define Yes(X) puts ( X ? "Yes" : "No")
#define YES(X) puts ( X ? "YES" : "NO")
#define FOR(i, a, b) for ( i = a, i < b ; i++)
#define REP(i, n) for ( i = 0; i < n ; i++)
#define DER(i, n) for ( i = 0; n ; i--)
#define max(a, b) (((a) > (b)) ? (a) : (b))
#define min(a, b) (((a) < (b)) ? (a) : (b))
#define max3(a, b, c) ((max((a), (b)) > (c)) ? max((a), (b)) : (c))
#define min3(a, b, c) ((min((a), (b)) < (c)) ? min((a), (b)) : (c))

/* const */
const int MOD = 1e9 + 7;

/* alias */
typedef long long ll;

/* function */
int cmp(const int *a, const int *b) {
	return *a - *b;
}
int cmp2(const int *a, const int *b) {
	return *b - *a;
}

//* main */

int main() {
	int n, a, b, i, p, x = 0, y = 0, z = 0;
	scanf("%d%d%d", &n, &a, &b);
	REP(i, n) {
		scanf("%d", &p);
		if (p <= a)x++;
		else if (p <= b)y++;
		else z++;
	}
	printf("%d", min3(x, y, z));
	return 0;
}