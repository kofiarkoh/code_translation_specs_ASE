#include<stdio.h>
int main() {
	long long int n, a[200005] = {}, result = 0;
	scanf("%lld", &n);
	for (int i = 0; i < n; i++) {
		scanf("%lld", &a[i]);
	}
	for (int i = 0; i < n-1; i++) {
		if (a[i] % 2 == 1&&a[i+1]>0) { a[i]--; a[i + 1]--; result++; }
		result += a[i] / 2;
	}
	result += a[n-1] / 2;
	printf("%lld", result);
}