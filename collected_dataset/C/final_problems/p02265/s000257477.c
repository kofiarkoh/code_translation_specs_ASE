// AOJ ALDS1_3_C Doubly Linked List
// 2018.5.1 bal4u

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

void skip(int k) { while (k--) gc(); }

void out(int n)
{
	int i;
	char ob[20];

	if (!n) { pc('0'); return; }
	i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
	while (i--) pc(ob[i]); 
}

typedef struct { int key; char f; } Q;
Q que[2000005]; int top, end;

int main()
{
	int n, i, t;
	char op;

	n = in(); top = 0, end = 1;
	while (n--) {
		op = gc();
		if (op == 'i') {
			skip(6);
			que[end].f = 1, que[end++].key = in();
		} else {
			skip(5);
			op = gc();
			if (op == 'F') {
				skip(5);
				while (que[end-1].f == 0) end--;
				end--;
			} else if (op == 'L') {
				skip(4);
				while (que[top+1].f == 0) top++;
				top++;
			} else {
				t = in();
				for (i = end-1; i > top; i--) {
					if (que[i].key == t && que[i].f) {
						que[i].f = 0;
						break;
					}
				}
			}
		}
	}
	t = 0;
	for (i = end-1; i > top; i--) if (que[i].f) {
		if (!t) t = 1; else pc(' ');
		out(que[i].key);
	}
	pc('\n');
	return 0;
}
