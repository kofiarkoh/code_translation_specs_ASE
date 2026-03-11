// AOJ 0349: Cyclic Sugoroku
// 2017.10.26

#include <stdio.h>

int q[100002], top, end;
int a[100002], f[100002];

int main()
{
	int n, i, t, ans;

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", &t);
		t = (t + i) % n;
		a[i] = t, f[t]++;
	}
	ans = 0, top = end = 0;
	for (i = 0; i < n; i++) if (!f[i]) q[end++] = i;
	while (top < end) {
		ans++;
		t = a[q[top++]];
		if (!--f[t]) q[end++] = t;
	}
	printf("%d\n", n - ans);
	return 0;
}