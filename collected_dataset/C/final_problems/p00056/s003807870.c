#include <stdio.h>

int main() {
	int prime[60000] = {1, 1};
	int prime_num[10000] = {0};
	int num;
	int i, j;
	int n;
	int ans;
	int temp;

	for(i = 2; i < 60000; i++)
		if(!prime[i])
			for(j = 2; i * j < 60000; j++)
				prime[i * j] = 1;

	for(i = 0, num = 0; i < 60000; i++) {
		if(!prime[i]) {
			prime_num[num] = i;
			num++;
		}
	}

	while(1) {
		ans = 0;
		scanf("%d", &n);
		if(n == 0)
			break;
		
		if(n % 2 == 1) {
			for(i = 0; prime_num[i] < n; i++)
				if(prime_num[i] == n - 2) {
					ans++;
					break;
				}
		}

		else {
			for(i = 0; prime_num[i] < n;i++) {
				temp = prime_num[i];
				for(j = i; temp + prime_num[j] <= n; j++) {
					if(temp + prime_num[j] == n)
						ans++;
				}
			}
		}
		
		printf("%d\n", ans);
	}

	return 0;
}