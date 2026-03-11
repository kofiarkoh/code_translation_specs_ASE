#include <stdio.h>

int main(void)
{
	int N;
	int a[100000];

	int max;
	int cnt, mcnt;

	int i;

	scanf("%d", &N);
	for (i = 0; i < N; i++){
		scanf("%d", &a[i]);
	}
	
	max = 1;
	for (i = 0; i < N; i++){
		if (a[i] > max) max = a[i];
	}

	cnt = 0;
	mcnt = 0;
	for (i = 0; i < N; i++){
		if (a[i] < max - 1) break;
		if (a[i] == max - 1) cnt++;
		else mcnt++;
	}

	if (i < N) {
		puts("No");
	} else if (max == N - 1 && mcnt == N){
		puts("Yes");
	} else if (cnt <= max - 1 && cnt >= max * 2 - N){
		puts("Yes");
	} else {
		puts("No");
	}

	return 0;
}
