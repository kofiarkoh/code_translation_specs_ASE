// Aizu 2490: Parentheses
// 2017.9.25 bal4u@uu

#include <stdio.h>

int main()
{
	int n, x, ans;
	char p[10];

	scanf("%d", &n); ans = 0;
	while (n-- > 0) {
		scanf("%s%d", p, &x);
		if (*p == '(') ans += x;
		else           ans -= x;
		if (ans < 0) { puts("NO"); return 0; }
	}
	puts(ans == 0 ? "YES" : "NO");
	return 0;
}