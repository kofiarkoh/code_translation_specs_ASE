#include <stdio.h>

#define MAX 1000100
#define MP 999983

int bool_prime[MAX] = {0};
int prime[MAX / 10] = {0};

void func_eratos(int array[], int size) {
	int i, j;

	array[0] = 1; array[1] = 1;
	for(i = 2; i < size; i++) {
		if(array[i] == 0) {
			for(j = 2; i * j < size; j++)
				array[i * j] = 1;
		}
	}
}

int mk_prime_array(int ans_array[], int array[], int max_prime_lim) {
	int i, j = 0;

	for(i = 0; i < max_prime_lim + 1; i++) {
		if(!array[i]) {
			ans_array[j] = i;
			j++;
		}
	}

	return j;
}

int b_search_index(int search, int array[], int bottom, int top) {
	int mid = (top + bottom) / 2;

	do {
		if(search < array[mid])
			top = mid;
		else
			bottom = mid;

		mid = (top + bottom) / 2;
	} while(bottom != mid);

	if(search > array[mid])
		mid++;

	return mid;
}

int func_premium(int m, int p, int prime_array[]) {
	int i = 0;
	int ans;

	while(prime_array[i]) i++;

	i = b_search_index(p - m, prime_array, 0, i - 1);

	for(ans = 0;prime_array[i] <= p + m && prime_array[i] <= MP; i++)
		ans++;

	return ans;
}

int main(void) {
	int n, m, p;
	int num_prime;
	int premium;
	int ans;
	int i;

	func_eratos(bool_prime, MAX);
	num_prime = mk_prime_array(prime, bool_prime, MAX);

	do {
		scanf("%d", &n);
		if(!n)
			break;

		for(i = 0, ans = 0; i < n; i++) {
			premium = 0;
			scanf("%d %d", &p, &m);
			premium += func_premium(m, p, prime); 
			ans += premium - 1;
		}

		printf("%d\n", ans);
	} while(1);
}