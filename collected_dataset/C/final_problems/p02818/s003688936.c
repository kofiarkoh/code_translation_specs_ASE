#include<stdio.h>
#include<string.h>

int main() {
	
	unsigned long A, B, C, K, ans;
	
	scanf("%lu %lu %lu", &A, &B, &K);

	if (A <= K) {
		C = K - A;
		A = 0;
		if (B <= C) {
			B = 0;
		}
		else {
			B = B - C;
		}
	}
	else {
		A = A - K;
	}

	printf("%lu %lu", A, B);

	return 0;
}

