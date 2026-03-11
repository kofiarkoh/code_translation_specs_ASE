#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

#define GYOU_MAX 256


long long combi(int n, int r) {
	if (r == 0) {
		return 1;
	}

	return (n - r + 1) * combi(n, r - 1) / r;
}


int main() {
	int n;
	scanf("%d", &n);

	int a[100000];
	for (int i = 0; i < n; i++) {
		scanf("%d", &a[i]);
	}

	long long int max = 0;
	int A, B;
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n - i; j++) {
			if (combi(a[i], a[j]) > max) {
				A = a[i];
				B = a[j];
				max = combi(a[i], a[j]);
			}
		}
	}

	//printf("%lld\n", max);
	printf("%d %d\n", A, B);

}
