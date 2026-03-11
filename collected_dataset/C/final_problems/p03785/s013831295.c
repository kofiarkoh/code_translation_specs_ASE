#include <stdio.h>
#include <stdlib.h>

int compare(const void* a, const void* b) {
	return *(long int*)a - *(long int*)b;
}

int main() {
	long int N, C, K, T[100000];
	scanf("%ld%ld%ld", &N, &C, &K);
	for (int i = 0; i < N; i++) scanf("%ld", &T[i]);
	qsort(T, N, sizeof(long int), compare);

	int car = 0, ans = 0, lim = 0;
	lim = T[0] + K, car++, ans++;
	//printf("%d:%d %d %d\n",T[0], car, ans, lim);
	for (int i = 1; i < N; i++) {
    	if (car == C||T[i]>lim) car = 0, ans++;
		if (car == 0)lim = T[i] + K;
		if (car < C && T[i] <= lim) car++;
		//printf("%d:%d %d %d\n",T[i], car, ans, lim);
	}
	printf("%ld", ans);
	return 0;
}