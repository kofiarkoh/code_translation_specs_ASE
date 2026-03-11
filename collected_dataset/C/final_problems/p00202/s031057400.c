#include <stdio.h>
#include <stdbool.h>
#include <math.h>

#define REP(i, n) for((i) = 0; (i) < (n); (i)++)

int price[30], meal_num;
int dp[1000002];

int get_max_total(int sup)
{
	int i, max = 0;

	if(dp[sup] >= 0) {
		return dp[sup];
	}
	
	REP(i, meal_num) {
		if(price[i] < sup) {
			int partial_max = price[i] + get_max_total(sup - price[i]);

			if(partial_max > max) max = partial_max;
		}
	}

	dp[sup] = max;

	return max;
}

inline int is_prime_number(int n)
{
	int i;
	for(i = 2; i <= (int)sqrt(n+1); i++) {
		if(n % i == 0) return false;
	}
	return true;
}

int main(void)
{
	while(1) {
		int i, ans, budget;

		scanf("%d%d", &meal_num, &budget);

		if(meal_num == 0 && budget == 0) {
			break;
		}

		REP(i, meal_num) {
			scanf("%d", &price[i]);
		}

		REP(i, 1000002) {
			dp[i] = -1;
		}

		ans = budget + 1;

		while(1) {
			ans = get_max_total(ans);

			if(ans == 0) break;

			if(is_prime_number(ans)) {
				printf("%d\n", ans);
				break;
			}
		}

		if(ans == 0) {
			printf("NA\n");
		}
	}

	return 0;
}