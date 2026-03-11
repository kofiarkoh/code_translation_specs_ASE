#include <stdio.h>

int main(void) {
	int n, ans1=0, ans2=0;
	int p[51]={ 0 }, seq[51]={ 0 };

	scanf("%d", &n);
	for (int i = 0; i < n; ++i) {
		scanf("%d", &p[i]);
	}

	for (int i = 0; i < n; ++i) {
		seq[i] = i + 1;
	}
	for (int i = 0; i < n;++i) {
		if (seq[i] == p[i]) {
			++ans1;
		}
		else {
			++ans2;
		}

	}
	printf(ans1 == 2 || ans2 == 2 ||ans1==0||ans2==0 ? "YES" : "NO");

	return 0;
}