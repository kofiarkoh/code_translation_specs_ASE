#include <stdio.h>

int n;
int wall[100];
int cnt[101];

int delete(int now) {
	int len;

	wall[now] = 0;

	len = 1;
	if (wall[(now + 1) % n])
		len += delete((now + 1) % n);
	if (wall[(now - 1 + n) % n])
		len += delete((now - 1 + n) % n);

	return len;
}

int main(void) {
	int i;
	int m;

	scanf("%d %d", &n, &m);

	while (m--) {
		int a, l;

		scanf("%d %d", &a, &l);

		for (i = 0; i < l; i++)
			wall[(a + i) % n] = 1;
	}

	for (i = 0; i < n; i++)
		if (wall[i]) {
			cnt[delete(i)]++;
		}

	for (i = 100; i >= 1; i--)
		if (cnt[i])
			printf("%d %d\n", i, cnt[i]);

	return 0;
}
