#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

#define INF 1000000000
 
#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

int compare(const void *a, const void *b) {
	return *(int*)a - *(int*)b;
}

int main() {
	int N;
	int ans = 0;
	int a[100];
	int i, j;
	
	scanf("%d", &N);
	for(i=0;i<N;i++) {
		scanf("%d", &a[i]);
	}
	
	qsort(a, N, sizeof(int), compare);
	
	if(a[0] < (a[N-1]+1)/2) {
		printf("Impossible\n");
		return 0;
	}
	
	if(N - 1 < a[N-1]) {
		printf("Impossible\n");
		return 0;
	}
	
	if(a[0] == a[2]) {
		printf("Impossible\n");
		return 0;
	}
	
	if(N > 2 && a[0] == 1 && a[1] == 1) {
		printf("Impossible\n");
		return 0;
	}
	
	if(a[N-1] != a[N-2]) {
		printf("Impossible\n");
		return 0;
	}
	
	printf("Possible\n");
	
	return 0;
}