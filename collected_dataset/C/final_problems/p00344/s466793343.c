// AOJ 0349: Cyclic Sugoroku
// 2017.10.26 bal4u@uu
// 2017.11.14

#include <stdio.h>

int q[100002], top, end;
int a[100002], f[100002];

char buf[1100005], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int n, i, t, ans;

//	scanf("%d", &n);
	fgets(p=buf, 10, stdin), n = getint();

	fgets(p=buf, sizeof(buf), stdin);
	for (i = 0; i < n; i++) {
		t = i + getint(), p++;
		if (t >= n) t %= n;
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