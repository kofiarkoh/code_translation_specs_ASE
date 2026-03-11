// AOJ DPL_3_C Largest Rectangle in a Histogram
// 2018.5.12 bal4u
 
#include <stdio.h>
#include <stdlib.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
 
int in()
{
    int n = 0, c = gc();
    do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
    return n;
}

typedef struct { int lp, h; } STACK;
STACK *stack; int top;

long long maxRectInHistogram(int n, int *h)
{
	int i;
	long long s, ans;
	
	stack = malloc(sizeof(STACK) * (n+5));
	h[n] = 0;
	
	ans = 0;
	stack[0].lp = -1, stack[0].h = -1, top = 0;
	for (i = 0; i <= n; i++) {
		if (h[i] == stack[top].h) continue;
		if (h[i] > stack[top].h) {
			stack[++top].lp = i, stack[top].h = h[i];
		} else {
			while (h[i] < stack[top].h) {
				s = (long long)stack[top].h*(i-stack[top].lp);
				top--;
				if (s > ans) ans = s;
			}
			stack[++top].h = h[i];
		}
	}
	free(stack);
	return ans;
}

int h[100002];

int main()
{
	int N, i;

	N = in();
	for (i = 0; i < N; i++) h[i] = in();

	printf("%lld\n", maxRectInHistogram(N, h));
	return 0;
}

