// AOJ 2348: Testing Circuits
// 2017.12.15 bal4u@uu

#include <stdio.h>
#include <ctype.h>

#define M  1000000007
#define BASE 100
#define X	 0
#define LP	 20		// (
#define RP   20		// )
#define NOT  10		// ~
#define AND  2		// &
#define OR   1		// |

typedef struct { char s; int a, t; } T;
T stack[1000002]; int top;
char  buf[1000005], *p;

int main()
{
	long long a, t;

	fgets(buf, sizeof(buf), stdin);
	top = 0, stack[top++].s = BASE;
	a = t = 1;
	for (p = buf; ; p++) {
		if      (*p == '(') stack[top++].s = LP;
		else if (*p == '&') stack[top++].s = AND;
		else if (*p == '|') stack[top++].s = OR;
		else if (*p == '~') stack[top++].s = NOT;
		else if (*p == 'x') {
			p++; while (*p >= '0' && *p <= '9') p++; p--;
			a = 2, t = 1;
			while (1) {
				while (stack[top-1].s == NOT) top--;
				if (stack[top-1].s != AND) {
					stack[top].s = X, stack[top].a = (int)a, stack[top++].t = (int)t;
					break;
				}
				top-=2;
				t = (t * stack[top].t) % M; 
				a = (a * stack[top].a) % M;
			}
		} else if (*p == ')' || *p < ' ') {
			a = 0, t = 0;
			while (1) {
				if (*p < ' ' && stack[top-1].s == BASE) goto done;
				if (stack[top-1].s == LP) {
					top--;
					while (1) {
						if (*p < ' ' && stack[top-1].s == BASE) goto done;
						while (stack[top-1].s == NOT) {
							top--;
							t = a - t;
							if (t < 0) t += M;
						}
						if (stack[top-1].s != AND) {
							stack[top].s = X, stack[top].a = (int)a, stack[top++].t = (int)t;
							goto next;
						}
						top -= 2;
						if (!a) a = t = 1;
						t = (t * stack[top].t) % M;
						a = (a * stack[top].a) % M;
					}
				}
				if (stack[top-1].s == X) {
					top--;
					if (!a) a = t = 1;
					t = (t * stack[top].t) % M;
					a = (a * stack[top].a) % M;
				}
				if (stack[top-1].s == OR) {
					long long a2, t2, aa, tt;
					top -= 2;
					t2 = stack[top].t; 
					a2 = stack[top].a;
					aa = (a*a2) % M;
					tt = (aa - (a-t)*(a2-t2)) % M; if (tt < 0) tt += M;
					a = aa,	t = tt;
				}
			}
		}
next:;
	}
done:
	printf("%lld\n", t);
	return 0;
}