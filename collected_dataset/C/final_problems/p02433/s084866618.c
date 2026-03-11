// AOJ ITP2_1_C List
// 2019.3.8 bal4u

#include <stdio.h>
#include <stdlib.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in()    // 整数入力（マイナス値でもOK）
{
	int n = 0, c = gc();
	if (c == '-') {
		c = gc();
		do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
		return -n;
	}
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)   // 整数表示（マイナス値でもOK）、最後に改行あり
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		if (n < 0) pc('-'), n = -n;
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

#define END 1000000001
typedef struct { int x, pre, nxt; } T;
T tbl[500002]; int top, cur, sz;

int main()
{
	int q, c, x, pre, nxt;

	sz = 1, tbl[0].pre = tbl[0].nxt = -1, tbl[0].x = END, top = cur = 0;
	q = in();
	while (q--) {
		c = gc() & 0xf, gc();
		if (c == 0) {
			x = in();
			nxt = cur, pre = tbl[cur].pre, cur = sz++;
			tbl[cur].x = x;
			if (pre < 0) tbl[cur].pre = -1, top = cur;
			else tbl[cur].pre = pre, tbl[pre].nxt = cur;
			if (nxt < 0) tbl[cur].nxt = -1;
			else tbl[cur].nxt = nxt, tbl[nxt].pre = cur;
		}
		else if (c == 1) {
			x = in();
			if (x < 0) { x = -x; while (x--) cur = tbl[cur].pre; }
			else while (x--) cur = tbl[cur].nxt;
		}
		else {   // c == 2
			pre = tbl[cur].pre, nxt = tbl[cur].nxt;
			if (pre < 0) tbl[nxt].pre = -1, cur = top = nxt;
			else if (nxt < 0) tbl[pre].nxt = -1, cur = pre;
			else tbl[pre].nxt = nxt, tbl[nxt].pre = pre, cur = nxt;
		}
	}
	cur = top;
	while (tbl[cur].x != END) {
		out(tbl[cur].x);
		cur = tbl[cur].nxt;
	}
	return 0;
}
