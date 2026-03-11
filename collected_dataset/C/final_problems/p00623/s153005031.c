// AOJ 1037 Midnight Teatime
// 2018.2.3 bal4u@uu

#include <stdio.h>
#include <string.h>

int info[10];
char tree[100], *p;
int buf[10][16], sz;

int *calc()
{
	int i, j, k;
	int *left, *right, *ans;

	ans = buf[sz++];
	if (*p == '(') {
		p++, left = calc(), p++, right = calc(), p++;
		for (i = 0; i < 16; i++) for (j = 0; j < 16; j++) {
			if (k = left[i] * right[j]) {
				ans[i & j] += k;
				ans[i | j] += k;
				ans[i ^ j] += k;
			}
		}
	} else ans[info[*p++ -'1']] = 1;
	return ans;
}

int main()
{
	int n, i, j;
	int *ans;
	char s[20];

	while(fgets(tree,100, stdin) && *tree != 'E') {
		memset(buf, 0, sizeof(buf));
		memset(info, 0, sizeof(info));

		fgets(s, 5, stdin), n = *s & 0xf;
		for (i = 0; i < n; i++) {
			fgets(p=s, 20, stdin);
			for (j = 0; j < 4; j++, p+=2) if (*p & 1) info[i] |= (1<<j);
		}

		p = tree, sz = 0, ans = calc();
		printf("%d\n", ans[15]);
	}
	return 0;
}
