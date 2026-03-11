// AOJ DSL_2_G RSQ and RAQ
// 2018.5.18 bal4u

#include <stdio.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(long long n)
{
	int i;
	char ob[40];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

#define MAX_N 100005

int s, t, x;
long long ans;

typedef struct { int l, r; long long w; int f; } NODE;
NODE node[1<<19];

void segTree(int k, int l, int r) 
{
	int m;

	node[k].l = l, node[k].r = r;
	if (node[k].l == node[k].r) return;
	m = (l + r) >> 1;
    segTree( k << 1, l, m);
    segTree((k << 1) | 1, m+1, r);
}

void down(int k) 
{
    node[ k << 1     ].f += node[k].f;
    node[(k << 1) | 1].f += node[k].f;
    node[ k << 1     ].w += (long long)node[k].f * (node[ k << 1     ].r - node[ k << 1     ].l + 1);
    node[(k << 1) | 1].w += (long long)node[k].f * (node[(k << 1) | 1].r - node[(k << 1) | 1].l + 1);
    node[k].f = 0;
}

void ask_point(int k)
{
	int m;

    if (node[k].l == node[k].r) {
        ans = node[k].w;
        return;
    }
    if (node[k].f) down(k);
    m = (node[k].l + node[k].r) >> 1;
    if (s <= m) ask_point(k << 1);
    else ask_point((k << 1) | 1);
}

void change_point(int k) 
{
	int m;

    if (node[k].l == node[k].r) {
        node[k].w += x;
        return;
    }
    if (node[k].f) down(k);
    m = (node[k].l + node[k].r) >> 1;
    if (s <= m) change_point(k << 1);
    else change_point((k << 1) | 1);
    node[k].w = node[k << 1].w + node[(k << 1) | 1].w;
}

void ask_interval(int k)
{
	int m;

    if (node[k].l >= s && node[k].r <= t) {
        ans += node[k].w;
        return;
    }
    if (node[k].f) down(k);
    m = (node[k].l + node[k].r) >> 1;
    if (s <= m) ask_interval(k << 1);
    if (m < t) ask_interval((k << 1) | 1);
}

void change_interval(int k) 
{
	int m;

	if (node[k].l >= s && node[k].r <= t) {
        node[k].w += (node[k].r - node[k].l + 1) * x;
        node[k].f += x;
        return;
    }
    if (node[k].f) down(k);
    m = (node[k].l + node[k].r) >> 1;
    if (s <= m) change_interval(k << 1);
    if (m < t) change_interval((k << 1) | 1);
    node[k].w = node[k << 1].w + node[(k << 1) | 1].w;
}

int main()
{
	int n, q, op;

	n = in();
    segTree(1, 1, n);

	q = in();
	while (q--) {
		op = gc() & 1, gc();
		if (op) {
			ans = 0;
			s = in(), t = in();
			if (s == t) ask_point(1);
			else ask_interval(1);
			out(ans);
		} else {
			s = in(), t = in(), x = in();
			if (s == t) change_point(1);
			else change_interval(1);
		}
	}
	return 0;
}

