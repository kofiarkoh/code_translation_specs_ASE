#include <stdio.h>

int N;
int a[100000];
void core() {
	int N;
	scanf("%d", &N);
	for (int i = 0; i < N; i++) {
		scanf("%d", &a[i]);
	}
	int flag;
	while (1) {
		flag = 0;
		if (a[0] < 0) {
			flag = -1;
			break;
		}
		int t = 0;
		int d = 1;
		for (int i = 1; i < N; i++) {
			if (a[i] < 0) {
				flag = -1;
				break;
			} else if (a[i] > 0) {
				flag = 1;
			}
			if (a[i - 1] > a[i]) {
				t = i;
				d = (a[i - 1] - a[i]) / N + 1;
				break;
			}
		}

		if (flag <= 0) {
			break;
		}

		for (int i = 0; i < N; i++) {
			a[(t + i) % N] -= (i + 1) * d;
		}
	}
	if (flag == 0) {
		printf("YES\n");
	} else {
		printf("NO\n");
	}
}

int main() {
	core();
	return 0;
}

