// AOJ 2596: Points and Lines
// 2017.12.20 bal4u@uu

#include <stdio.h>
#include <ctype.h>
#include <math.h>

#define BASE  100
#define P1	  1
#define P2    2
#define LP	  20		// (
#define RP    21		// )
#define OP    10		// @

typedef struct { double x, y; } PP;
typedef struct { PP s, e; } LINE;

typedef struct { int k; PP p1, p2; } S;
S stack[110]; int top;
char buf[110];

void intersection(PP *ans, LINE *u, LINE *v)
{
	double t1, t2, t3, t4;
	double bo, ua;

	t1 = (u->s.y - v->s.y)*(v->s.x - v->e.x) - (u->s.x - v->s.x)*(v->s.y - v->e.y);
	t2 = (u->e.y - v->s.y)*(v->s.x - v->e.x) - (u->e.x - v->s.x)*(v->s.y - v->e.y);
	t3 = (v->s.y - u->s.y)*(u->s.x - u->e.x) - (v->s.x - u->s.x)*(u->s.y - u->e.y);
	t4 = (v->e.y - u->s.y)*(u->s.x - u->e.x) - (v->e.x - u->s.x)*(u->s.y - u->e.y);

	bo = (v->e.y - v->s.y)*(u->e.x - u->s.x) - (v->e.x - v->s.x)*(u->e.y - u->s.y);
	ua = (v->e.x - v->s.x)*(u->s.y - v->s.y) - (v->e.y - v->s.y)*(u->s.x - v->s.x);
	ans->x = u->s.x + ua/bo * (u->e.x - u->s.x);
	ans->y = u->s.y + ua/bo * (u->e.y - u->s.y);
}

void mirrorPoint(PP *ans, PP *p, LINE *u)
{
	double A, B, C;   // Ax + By + C = 0 

	A = u->s.y - u->e.y, B = u->e.x - u->s.x, C = u->e.y*u->s.x - u->e.x*u->s.y;
	ans->x = p->x - 2 * A * (A * p->x + B * p->y + C) / (A*A + B*B);
	ans->y = p->y - 2 * B * (A * p->x + B * p->y + C) / (A*A + B*B);
}

char *getInt(int *a, char *p)
{
	int x = 0;
	if (*p == '-') {
		p++; while (isdigit(*p)) x = 10*x + (*p++ & 0xf);
		x = -x;
	} else while (isdigit(*p)) x = 10*x + (*p++ & 0xf);
	*a = x;
	return p;
}

char *getPP(PP *a, char *p)
{
	int t;
	p = getInt(&t, p) + 1;  // skip ,
	a->x = t;
	p = getInt(&t, p);
	a->y = t;
	return p;
}

void calc(S *a, int i, int j)
{
	LINE u, v;

	if (stack[i].k == P1 && stack[j].k == P1) {
		a->p1 = stack[i].p1, a->p2 = stack[j].p1;
		a->k = P2;
	} else if (stack[i].k == P2 && stack[j].k == P1) {
		u.s = stack[i].p1, u.e = stack[i].p2;
		mirrorPoint(&(a->p1), &stack[j].p1, &u);
		a->k = P1;
	} else if (stack[i].k == P1 && stack[j].k == P2) {
		u.s = stack[j].p1, u.e = stack[j].p2;
		mirrorPoint(&(a->p1), &stack[i].p1, &u);
		a->k = P1;
	} else {
		u.s = stack[i].p1, u.e = stack[i].p2;
		v.s = stack[j].p1, v.e = stack[j].p2;
		intersection(&(a->p1), &u, &v);
		a->k = P1;
	}
}

int main()
{
	char *p;
	S ans;

	while (fgets(buf, 105, stdin) && *buf != '#') {
		top = 0, stack[top++].k = BASE, stack[top++].k = BASE, stack[top++].k = BASE;
		for (p = buf; *p > ' ' ; p++) {

			if (*p == '(') {
				if (*(p+1) == '-' || isdigit(*(p+1))) {
					p = getPP(&stack[top].p1, p+1);
					stack[top].k = P1;
					ans = stack[top++];
					if (stack[top-2].k == OP) {
						top -= 3;
						calc(&ans, top, top+2);
						stack[top++] = ans;
					}
				} else stack[top++].k = LP;
			} else if (*p == '@') {
				if (stack[top-1].k < OP && stack[top-2].k == OP && stack[top-3].k < OP) {
					calc(&ans, top-3, top-1);
					top -= 3;
					stack[top++] = ans;
				}
				stack[top++].k = OP;
			} else if (*p == ')' || *p < ' ') {
				while (1) {
					if (*p < ' ' && stack[top-1].k == BASE) goto done;
					if (stack[top-1].k == LP) {
						top--;
						while (1) {
							if (*p < ' ' && stack[top-1].k == BASE) goto done;
							if (stack[top-1].k != OP) {
								stack[top++] = ans;
								goto next;
							}
							stack[--top] = ans;
							top--;
							calc(&ans, top, top+1);
						}
					}
					if (stack[top-1].k == OP) {
						stack[--top] = ans;
						top--;
						calc(&ans, top, top+1);
					} else 	ans = stack[--top];
				}
			}
next:;
		}
done:
		printf("%.8lf %.8lf\n", ans.p1.x, ans.p1.y);
	}
	return 0;
}