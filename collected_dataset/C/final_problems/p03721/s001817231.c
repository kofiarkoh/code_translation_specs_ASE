#include <stdio.h>
#include <stdlib.h>
#define max_size (int)(1e5 + 1)

int main(){
	int N, K, a, b, i;
	scanf("%d%d", &N, &K);
	int *anum = (int *)malloc(sizeof(int) * max_size);
	for(i = 0; i < max_size; i++){
		anum[i] = 0;
	}
	for(i = 0; i < N; i++){
		scanf("%d%d", &a, &b);
		anum[a] += b;
	}
	int sum = 0;
	for(i = 0; sum < K; i++){
		sum += anum[i];
	}
	printf("%d\n", i - 1);
	return 0;
}