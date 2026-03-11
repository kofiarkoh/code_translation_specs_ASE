// AOJ 2720: Identity Function
// 2017.10.9 bal4u@uu

#include <stdio.h>
#include <math.h>

#define SIZE  25
int size;
int factor[SIZE];

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
  int i, d, sw;
  int *pp;

  size = 0;
  if ((n & 1) == 0) {
    factor[size++] = 2;
    do n >>= 1LL;
    while ((n & 1) == 0);
  }
  for (pp = ptbl; n > 1 && *pp > 0; pp++) {
    if (n % *pp) continue;
    d = *pp;
    factor[size++] = d;
    do n /= d;
    while (n % d == 0);
  }
  if (n > 1) {
    int b = (int)sqrt((double)n);
    for (i = 1009, sw = 0; n > 1; ) {
      if (i > b) {
        factor[size++] = (int)n;
        break;
      }
      if (n % i == 0) {
        factor[size++] = i;
        do n /= i;
        while (n % i == 0);
      }
      if (sw) { i += 2, sw = 0; }
      else    { i += 4, sw = 1; }
    }
  }
}

long long gcd(long long a, long long b)
{
	long long r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

long long lcm(long long a, long long b)
{
	return a/gcd(a, b)*b;
}

long long big_mod(int b, long long p, long long m)
{
   long long s = 1;
   long long d;

   d = b % m;
   while (p > 0) {
      if (p & 1) s = (s * d) % m;
      p >>= 1;
      d = (d * d) % m;
   }
   return s;
}

int main()
{
	int n, i, i2;
	long long L, a, k;

	scanf("%d", &n);
	for (i = 2; ; i++) {
		if ((i2 = i*i) > n) break;
		if (n % i2 == 0) goto NG;
	}

	prime_factor(n);
	L = factor[0]-1;
	for (i = 1; i < size; i++) L = lcm(L, factor[i]-1);
	if (L == 1) { puts("1"); return 0; }
	if (gcd(n, L) != 1) goto NG;

	prime_factor(L);
	for (a = L, i = 0; i < size; i++) a = a/factor[i]*(factor[i]-1);

	for (k = 1; k <= a; k++) {
		if (a % k == 0 && big_mod(n, k, L) == 1) {
			printf("%lld\n", k);
			return 0; 
		}
	}
NG: puts("-1");
	return 0;
}