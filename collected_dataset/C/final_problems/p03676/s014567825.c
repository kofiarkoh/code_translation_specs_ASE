//#include "oj/atcoder.h"
//#include "std.h"
#include <stdlib.h>
#include <stdbool.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#include <assert.h>
#include <float.h>

#define MIN(a,b) ((a)<(b)?(a):(b))
#define MAX(a,b) ((a)>(b)?(a):(b))
#define SWAP(T,a,b) do { T _tmp=a; a=b; b=_tmp; } while (0)

//#include "oj/io.h"

void print_ai(int *a, int n) {
	for (int i=0; i<n; i++) {
		if (i) printf(" ");
		printf("%d", a[i]);
	}
	printf("\n");
}

int scan_ai(int *a, int n) {
	for (int i=0; i<n; i++) {
		if (scanf("%d", &a[i]) != 1)
			return i;
	}
	return n;
}

//#include "oj/debug.h"

void print_hr() {
	printf("---------------------------\n");
}



#define T long
#define MODULUS 1000000007
//#include "math/integer/mod.h"
//#define MODULUS 1000000007

//#include "math/arithmetic/mod.h"

//#include "math/arithmetic/gcd.h"
#ifndef MATH_GCD_H
#define MATH_GCD_H

/* ax+by=gcd(a,b) */
T gcd_ext(T a, T b, T *x, T *y) {
	if (a == 0) {
		*x = 0;
		*y = 1;
		return b;
	}
	T x1, y1;
	T gcd = gcd_ext(b%a, a, &x1, &y1);
	*x = y1 - (b/a) * x1;
	*y = x1;
	return gcd;
}

#endif


static inline T mod_norm(T a, T m) {
	return (a % m + m) % m;
}

static inline T mod_add(T a, T b, T m) {
	return (a + b) % m;
}

static inline T mod_sub(T a, T b, T m) {
	return (a + m - b) % m;
}

static inline T mod_mul(T a, T b, T m) {
	return a * b % m;
}

static inline T mod_div(T a, T b, T m) {
	T x, y;
	T g = gcd_ext(b, m, &x, &y);
	T d = (g!=1) ? 0 : mod_norm(x, m);
	return (a * d) % m;
}

//n^k
T mod_power(T a, T b, T m) {
	T p = a, r = 1;
	while (b) {
		if (b % 2)
			r = (r * p) % m;
		b /= 2;
		p = p * p % m;
	}
	return r;
}

//n!
T mod_factorial(int n, T m) {
	T ct = 1;
	for (int i=2; i<=n; i++)
		ct = (ct * i) % m;
	return ct;
}


#define add(a, b) (mod_add(a, b, MODULUS))
#define sub(a, b) (mod_sub(a, b, MODULUS))
#define mul(a, b) (mod_mul(a, b, MODULUS))
#define div(a, b) (mod_div(a, b, MODULUS))

#define power(a, p)     (mod_power(a, p, MODULUS))
#define factorial(n)    (mod_factorial(n, MODULUS))
//#include "math/combinatorics/count.h"
//#include "math/integer/mod.h"

//P(n,k)
T permutations(int n, int k) {
	if (k > n)
		return 0;
	T ct = 1;
	for (int i=0; i<k; i++)
		ct = mul(ct, n-i);
	return ct;
}


//C(n,k)
T combinations(int n, int k) {
	T ct = div(permutations(n, k), factorial(k));
	return ct;
}

void combinations_matrix(int n, int r, T c[n+1][n+1]) {
	// C(n,r) = C(n-1,r-1) + C(n-1,r)
	// where C(n,0) = C(n,n) = 1
	for(int j=0; j<=r; j++)
		c[0][j] = 0;
	for(int i=0; i<=n; i++)
		c[i][0] = 1;
	for(int i=1; i<=n; i++) {
		for(int j=1; j<=r; j++) {
			if (i == j) {
				// C(n,n) = 1
				c[i][j] = 1;
			}
			else if (j > i) {
				c[i][j] = 0;
			}
			else {
				c[i][j] = add(c[i-1][j-1], c[i-1][j]);
			}
		}
	}
}


int main() {
	int n;
	while (scanf("%d", &n)==1) {
		n++;
		int a[n];
		int b[n];
		scan_ai(a, n);

		T c[n+1][n+1];
		combinations_matrix(n, n, c);

		memset(b, -1, sizeof(b));
		int d;
		for (int i=0; i<n; i++) {
			if (b[a[i]] >= 0) {
				d = i - b[a[i]];
				break;
			} else {
				b[a[i]] = i;
			}
		}

		for (int k=1; k<=n; k++) {
			long r = c[n][k] - c[n-d-1][k-1];
			printf("%ld\n", r);
		}
	}
}
