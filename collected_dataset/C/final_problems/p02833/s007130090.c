#include<stdio.h>
int main(void)
{
	long long int n = 0,ans = 0,count = 1;
	scanf("%lld", &n);
	
	if (n % 2 != 0) {
		printf("0\n");
	}
	else {
		int under = 10;
		while (1) {
			if (under * count > n) {
				break;
			}
			ans += n / (under * count);
			count *= 5;
		}
		printf("%lld\n", ans);
	}
	return 0;
}