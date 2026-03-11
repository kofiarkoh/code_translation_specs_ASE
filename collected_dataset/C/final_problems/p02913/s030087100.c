#include <stdio.h>

int N;
char S[5005];

int match(int len) {
	int l1,l2;
	int i;

	int ret;

	for (l1 = 0; l1 <= N - len; l1++) {
		for (l2 = l1 + len; l2 <= N - len; l2++) {
			ret = 1;
			if (l2 + len - 1 < N) {
				for (i = 0; i < len; i++) {
					if (S[l1 + i] != S[l2 + i]) {
						ret = 0;
						break;
					}
				}
				if (ret == 1) { return 1; }
			}
		}
	}
	return 0;
}

int main() {


	scanf("%d",&N);

	scanf("%s",S);

	int l, r, mid, ans;

	ans = 0;
	l = 0;
	r = N;


	while (l < r) {
		mid = (l + r) / 2;
		if (match(mid) == 1) {
			l = mid + 1;
			if (ans < mid) { ans = mid; }
		}
		else {
			r = mid;
		}
	}

	printf("%d\n",ans);

	return 0;
}