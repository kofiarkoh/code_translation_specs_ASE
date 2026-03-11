// AOJ 2428: Lost Number
// 2017.10.19 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 1024

char q[10002][102]; int top;

char *expr();
char *factor(char *s, char *p, int *z, int *op)
{
	int x, k;

	if (*p == '(') {
		k = 0;
		if ((p = expr(s, p+1, &x, &k)) == NULL) return NULL;
		if (!k || (*p != ')' && *p != '.')) return NULL;
		p++;
		*op = 0;
	} else if (*p == '1' || *p == '0' || *p == '.') {
		x = 0;
		while (*p == '1' || *p == '0') x = (x << 1) + (*p++ - '0');
		while (*p == '.') {
			strcpy(q[top], s), q[top++][p-s] = '1';
			strcpy(q[top], s), q[top++][p-s] = '0';
			strcpy(q[top], s), q[top++][p-s] = '+';
			strcpy(q[top], s), q[top++][p-s] = '-';
			strcpy(q[top], s), q[top++][p-s] = '*';
			strcpy(q[top], s), q[top++][p-s] = '(';
			strcpy(q[top], s), q[top++][p-s] = ')';
			x = (x << 1) + 1, p++;
		}
		while (*p == '1' || *p == '0') x = (x << 1) + (*p++ - '0');
		*op = 0;
	} else return NULL;
	if (x < 0 || x >= MAX) return NULL;
	*z = x;
	return p;
}

char *term(char *s, char *p, int *z, int *op)
{
	int x, y, k = 0;

	if ((p = factor(s, p, &x, &k)) == NULL) return NULL;
	while (1) {
		if (*p == '.') {
			strcpy(q[top], s), q[top++][p-s] = '*';
			strcpy(q[top], s), q[top++][p-s] = '+';
			strcpy(q[top], s), q[top++][p-s] = '-';
			break;
		} else if (*p == '*') ;
		else break;
		if ((p = factor(s, p+1, &y, &k)) == NULL) return NULL;
		x *= y, k = 1;
		if (x < 0 || x >= MAX) return NULL;
	}
	*z = x;
	*op = k;
	return p;
}

char *expr(char *s, char *p, int *z, int *op)
{
	int x, y, t, k = 0;
	
	if ((p = term(s, p, &x, &k)) == NULL) return NULL;
	while (1) {
		if (*p == '.') {
			strcpy(q[top], s), q[top++][p-s] = '*';
			strcpy(q[top], s), q[top++][p-s] = '+';
			strcpy(q[top], s), q[top++][p-s] = '-';
			break;
		} else if (*p == '+') t = 1;
		else if (*p == '-') t = 0;
		else break;
		if ((p = term(s, p+1, &y, &k)) == NULL) return NULL; 
		if (t) x += y;
		else   x -= y;
		k = 1;
		if (x < 0 || x >= MAX) return NULL;
	}
	*z = x;
	*op = k;
	return p;
}

int main()
{
	int n, op, ans;
	char *p;

	scanf("%s", q[0]);
	ans = -1, top = 1;
	while (top > 0) {
		top--;
		if ((p = expr(q[top], q[top], &n, &op)) == NULL) continue;
		if (*p) continue;
		if (n >= 0 && n < MAX && n > ans) ans = n;
	}
	printf("%d\n", ans);
	return 0;
}