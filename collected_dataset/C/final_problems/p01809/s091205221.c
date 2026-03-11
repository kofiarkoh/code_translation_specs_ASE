// AOJ 2706: Let's Solve Geometric Problems
// 2017.10.9 bal4u@uu

#include <stdio.h>
#include <math.h>

#define SIZE  20
int factor[SIZE], size;

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

void prime_factor(int n)
{
  int i, d;
  int sw;
  int *pp;

  size = 0;
  if ((n & 1) == 0) {
    factor[size++] = 2;
    do n >>= 1;
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
    d = (int)sqrt(n);
    for (i = 1009, sw = 0; n > 1; ) {
      if (i > d) {
        factor[size++] = n;
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

int gcd(int a, int b)
{
	int r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

int main()
{
	int p, q, i, ans;

	scanf("%d%d", &p, &q);
	i = gcd(p, q), p /= i, q /= i;
	prime_factor(q);
	for (ans = 1, i = 0; i < size; i++) ans *= factor[i];
	printf("%d\n", ans);
	return 0;
}