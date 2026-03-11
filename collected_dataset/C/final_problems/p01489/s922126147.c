// Aizu 2372: IkaNumber
// 2017.10.30 bal4u@uu

#include <stdio.h>

#define INF 0x7fffffff
#define M 1000000007

void multiply(int f[2][2], int m[2][2])
{
	unsigned long long x, y, z, w;
	
	x =  (unsigned long long)f[0][0]*m[0][0] + (unsigned long long)f[0][1]*m[1][0];
	y =  (unsigned long long)f[0][0]*m[0][1] + (unsigned long long)f[0][1]*m[1][1];
	z =  (unsigned long long)f[1][0]*m[0][0] + (unsigned long long)f[1][1]*m[1][0];
	w =  (unsigned long long)f[1][0]*m[0][1] + (unsigned long long)f[1][1]*m[1][1];
 
	f[0][0] = (int)(x % M);
	f[0][1] = (int)(y % M);
	f[1][0] = (int)(z % M);
	f[1][1] = (int)(w % M);
}

void power(int f[2][2], int n)
{
	int m[2][2] = {{1,1},{1,0}};
	if( n == 0 || n == 1) return;
	power(f, n >> 1);
	multiply(f, f);
	if (n & 1) multiply(f, m);
}

int fib(int n)
{
	int f[2][2] = {{1,1},{1,0}};
//	if (n == 0) return 0;
	power(f, n-1);
	return f[0][0];
}
 
int main()
{
    unsigned long long K, l, r, m;
	int a, b;

	scanf("%llu", &K);

	l = 0;
	if (K < INF) r = K; else r = INF;
    while (l+1 < r) {
        m = (l + r) >> 1;
        if (m * (m + 1) < K) l = m; else r = m;
    }
	K -= l * (l + 1);
    a = (int)((l<<1) + (K-1)/r + 1);

    if (K > r) K -= r;
    if (K <= (r+1)>>1) b = (int)((K<<1) - 1);
    else b = (int)(r - (r & 1) - (((K - ((r+1)>>1) - 1))<<1));

	printf("%d\n", (int)(((long long)fib(a-b+2)*fib(b+1)) % M));
	return 0;
}