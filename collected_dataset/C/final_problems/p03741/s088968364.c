#include <stdio.h>

int main(void) {
	int i, n, sf;
	long long int a, sum, count = 0;

	scanf("%d%lld", &n, &sum);
	if (sum < 0) sf = 1;
	else		 sf = 0;
	
	for (i = 1; i < n; i++) {
		scanf("%lld", &a);
		
		if (!sf) {
			if (sum + a < 0) {
				sum = sum + a;
			}
			else {
				count = count + sum + a + 1;
				sum = -1;
			}
		}
		else {
			if (sum + a > 0) {
				sum = sum + a;
			}
			else {
				count = count - (sum + a - 1);
				sum = 1;
			}
		}
		sf = !sf;
	}
	printf("%lld\n", count);

	return 0;
}