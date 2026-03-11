#include <stdio.h>

#define ll long long

int main(int argc, char *argv[]) {
	int N;
	scanf("%d", &N);
	int A[81];
	for (int i = 0; i < N; i++) {
		scanf("%d", &A[i]);
	}
	A[N] = 100;

	ll money = 1000;
	for (int i = 0; i < N; i++) {
		if (A[i] < A[i + 1]) {
			ll stock = money / A[i];
			money += (A[i + 1] - A[i]) * stock;
		}
	}

	printf("%lld\n", money);
	return 0;
}
