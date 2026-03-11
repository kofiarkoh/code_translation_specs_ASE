#include <stdio.h>

long exp(int x, int n) {
	int i;
	long ans = 1;

	for(i = 0; i < n; i++)
		ans *= x;

	return ans;
}

long conv_oct(long x) {
	int i;
	long ans;
	long temp;
	int a[32];

	for(i = 0, temp = x; temp; i++) {
		a[i] = temp % 8;
		temp = temp / 8;
	}

	for(ans = 0; i > 0; i--)
		ans += a[i - 1] * exp(10, i - 1);

	return ans;
}

int main() {
	long input;
	long ans;
	int a[32];
	int i, temp;

	do {
		scanf("%ld", &input);
		if(!input)
			break;
		for(i = 0, temp = input; temp; i++) {
			a[i] = temp % 8;
			temp = temp / 8;
		}

		for(ans = 0; i > 0; i--) {
			if(a[i - 1] > 3)
				a[i - 1]++;
			if(a[i - 1] > 5)
				a[i - 1]++;

			ans += a[i - 1] * exp(10, i - 1);
		}

		printf("%ld\n", ans);
	} while(1);

	return 0;
}