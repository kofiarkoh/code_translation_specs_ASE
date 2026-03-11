#include <stdio.h>
int main() {
	int n, p, cnt = 0, ans = 0;
	scanf("%d", &n);
	for (int i = 0; i < n; i++) {
		scanf("%d", &p);
		if (i + 1 == p) {
			if (cnt != 0) {
				ans += p - cnt;
				cnt = 0;
			} else {
				cnt = p;
			}
		}
	}
	if (cnt != 0) ans++;
	printf("%d\n", ans);
	return 0;
}