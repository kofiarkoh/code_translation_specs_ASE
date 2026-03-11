// AOJ 2353 Four Arithmetic Operations
// 2017.11.27 bal4u@uu

#include <stdio.h>
#define LIM 2147483648LL
#define P   4294967311LL		// ???= 0x???10000000F??¬??¬, prime number??¬
#define P2  4294967309LL		// = P-2
#define M1	20					// 2^20 = 0x100000 = 1048576
#define M2  0xfffff				// = 2^20 -1

#define multi(x,y)	(((((x)*((y)>>M1)%P)<<M1)+(x)*((y)&M2))%P)

char buf[15], *p;
int getint()
{
	int n = 0;
	if (*p == '-') {
		p++; while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
		return -n;
	}
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

#if 0
long long multi(long long x, long long y)
{
	return (((x*(y>>M1)%P)<<M1) + x*(y&M2)) % P;
}
#endif

long long power(long long y, long long m)
{
   long long s = 1;
   long long d = y % P;

   while (m > 0) {
      if (m & 1) s = multi(s, d);
      m >>= 1;
      d = multi(d, d);
   }
   return s;
}

int main()
{
	int n, c, y;
	long long x;

	fgets(p=buf, 10, stdin), n = getint();
	x = 0;
	while (n--) {
		fgets(p=buf, 15, stdin);
		c = *p, p+=2, y = getint();
		if (c == '1') {
			x += y; if (x < 0) x += P; if (x >= P) x -= P;
		} else if (c == '2') {
			x -= y; if (x < 0) x += P; if (x >= P) x -= P;
		} else if (c == '3') {
			if (y < 0) x = multi(x, (P+y));
			else       x = (x*y) % P;
		} else {  // div
			if (y < 0) x = multi(x, power(P+y, P2));
			else       x = multi(x, power(y,   P2));
		}
	}
	if (x >= LIM) x -= P;
	printf("%lld\n", x); 
	return 0;
}