// AOJ 1531: Remainder Zero
// 2019.2.21 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

long long gcd(long long a, long long b)
{
	long long r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

long long lcm(long long a, long long b, long long lim)
{
	long long g = gcd(a, b);
	a /= g;
	if ((double)a * b > lim) return -1;
	return a * b;
}

int power[15], sz;
int ptbl[] = {
		 3,   5,   7,  11,  13,  17,  19,  23,  29,
   31,  37,  41,  43,  47,  53,  59,  61,  67,  71,
   73,  79,  83,  89,  97, 101, 103, 107, 109, 113,
  127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
  179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
  233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
  283, 293, 307, 311, 313, 317, 331, 337, 347, 349,
  353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
  419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
  467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
  547, 557, 563, 569, 571, 577, 587, 593, 599, 601,
  607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
  661, 673, 677, 683, 691, 701, 709, 719, 727, 733,
  739, 743, 751, 757, 761, 769, 773, 787, 797, 809,
  811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
  877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
  947, 953, 967, 971, 977, 983, 991, 997, 0 };

void prime_factor(long long n)
{
	int i, b, sw;
	int *pp, *p;

	pp = power;
	if ((n & 1) == 0) {
		do n >>= 1LL, (*pp)++;
		while ((n & 1) == 0);
		pp++;
	}
	for (p = ptbl; n > 1 && *p > 0; p++) {
		if (n % *p) continue;
		b = *p;
		do n /= b, (*pp)++;
		while (n % b == 0);
		pp++;
	}
	if (n > 1) {
		b = (int)sqrt((double)n);
		for (i = 1009, sw = 0; n > 1; ) {
			if (i > b) { *pp++ = 1; break; }
			if (n % i == 0) {
				do n /= i, (*pp)++;
				while (n % i == 0);
				pp++;
			}
			if (sw) i += 2, sw = 0;
			else    i += 4, sw = 1;
		}
	}
	sz = pp - power;
}

int N, M;
long long A[10], B[10];

int main()
{
	int i, ans;
	long long a_gcd, b_lcm;

	scanf("%d%d", &N, &M);
	for (i = 0; i < N; i++) scanf("%lld", A + i);
	for (i = 0; i < M; i++) scanf("%lld", B + i);

	a_gcd = A[0];
	for (i = 1; i < N; i++) a_gcd = gcd(a_gcd, A[i]);
	b_lcm = B[0];
	if (b_lcm > a_gcd) goto ZERO;
	for (i = 1; i < M; i++) {
		b_lcm = lcm(b_lcm, B[i], a_gcd);
		if (b_lcm < 0 || b_lcm > a_gcd) goto ZERO;
	}

//	printf("gcd %lld, lcm %lld\n", a_gcd, b_lcm);
	ans = 1;
	if (a_gcd < b_lcm) {
ZERO:	ans = 0;
	}
	else if (a_gcd == b_lcm) ;
	else {
		a_gcd /= b_lcm;
		prime_factor(a_gcd);
//		printf("sz=%d:", sz);
//		for (i = 0; i < sz; i++) printf(" %d", power[i]);
//		printf("\n");
		for (i = 0; i < sz; i++) ans *= 1 + power[i];
	}
	printf("%d\n", ans);
	return 0;
}
