#include<stdio.h>

long long int nibutan(long long int left, long long int right, long long int num) {
	long long int memo = (left + right) / 2;
	if (memo*(memo + 1) < num && (memo + 2)*(memo + 1) >= num) { return memo; }
	if (memo*(memo + 1) >= num && (memo + 2)*(memo + 1) >= num) { return nibutan(left, memo, num); }
	if (right - left == 1) { return right; }
	if (memo*(memo + 1) < num && (memo + 2)*(memo + 1) < num) { return nibutan(memo, right, num); }
}
long long int nibutan2(long long int left, long long int right, long long int num) {
	long long int memo = (left + right) / 2;
	if (memo*memo < num && (memo + 1)*(memo + 1) >= num) { return memo; }
	if (memo*memo >= num && (memo + 1)*(memo + 1) >= num) { return nibutan2(left, memo, num); }
	if (right - left == 1) { return right; }
	if (memo*memo < num && (memo + 1)*(memo + 1) < num) { return nibutan2(memo, right, num); }
}

int main() {

	long long int n, memo, r;
	long long int a[105], b[105];
	scanf("%lld", &n);
	for (int i = 0; i < n; i++) {
		scanf("%lld%lld", &a[i], &b[i]);
		memo = a[i] * b[i];


		if (a[i] == b[i]) {
			printf("%lld\n", 2 * (a[i] - 1));
		}
		else if (a[i] >= b[i] && nibutan(1, a[i], memo) >= nibutan2(1, a[i], memo)) {


			printf("%lld\n", 2 * nibutan(1, a[i], memo) - 1);
		}
		else if (a[i] < b[i] && nibutan(1, b[i], memo) >= nibutan2(1, b[i], memo)) {
			printf("%lld\n", 2 * nibutan(1, b[i], memo) - 1);

		}
		else if (a[i] >= b[i] && nibutan(1, a[i], memo) < nibutan2(1, a[i], memo)) {
			printf("%lld\n", 2 * nibutan(1, a[i], memo));
		}
		else if (a[i] < b[i] && nibutan(1, b[i], memo) < nibutan2(1, b[i], memo)) {
			printf("%lld\n", 2 * nibutan(1, b[i], memo));
		}
	}

}
