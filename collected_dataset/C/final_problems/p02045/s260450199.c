// AOJ 2945 Poison Swamp
// 2019.8.8 bal4u

#include <stdio.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in() {   // 非負整数の入力
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf); while ((c = gc()) >= '0');
	return n;
}

void out(int n) { // 非負整数の表示（出力）
	int i;
	char b[20];
	if (!n) pc('0');
	else {
		i = 0; while (n) b[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(b[i]);
	}
	pc('\n');
}

int N, A, B, C, D;
int x[105], y[105];
char f[105];

inline static int ABS(int x) { return (x >= 0)? x: -x; }
inline static int MIN(int a, int b) { return (a <= b)? a: b; }

int calc(int i) {
	int t, a;
	if (A <= x[i] && x[i] <= C) return MIN(ABS(y[i]-B), ABS(y[i]-D));
	if (B <= y[i] && y[i] <= D) return MIN(ABS(x[i]-A), ABS(x[i]-C));
	t = 100000;
	if ((a = ABS(x[i]-A) + ABS(y[i]-B)) < t) t = a;
	if ((a = ABS(x[i]-C) + ABS(y[i]-B)) < t) t = a;
	if ((a = ABS(x[i]-A) + ABS(y[i]-D)) < t) t = a;
	if ((a = ABS(x[i]-C) + ABS(y[i]-D)) < t) t = a;
	return t;
}

int main()
{
	int i, N, ans;
	
	while ((N = in())) {
		A = in(), B = in(), C = in(), D = in();
		memset(f, 0, N+1);
		for (i = 0; i <= N; i++) {
			x[i] = in(), y[i] = in();
			if (A <= x[i] && x[i] <= C && B <= y[i] && y[i] <= D) f[i] = 1;
		}
		
		ans = 0;
		for (i = 1; i <= N; i++) {
			if (f[i-1] && f[i]);
			else if (f[i-1]) ans += calc(i);
			else if (f[i]) ans += calc(i-1)-1;
			else ans += MIN(ABS(x[i-1]-x[i]) + ABS(y[i-1]-y[i]), calc(i-1)-1 + calc(i));
//			printf("[%d] ans=%d\n", i, ans);
		}
		out(ans);
	}
	return 0;
}
