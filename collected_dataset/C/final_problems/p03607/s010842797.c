#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int compare_llint_rise(const void *a, const void *b) {
	long long int A, B;
	A = *(long long int *)a;
	B = *(long long int *)b;
	if(A < B) {
		return -1;
	} else if (A > B){
		return 1;
	} else {
		return 0;
	}
}

int main(void) {
	int N;
	long long int *A;
	int i;
	int count;
	char buf[128], *ch;

	fgets(buf, sizeof(buf), stdin);
	ch = strtok(buf, " ");
	N = atoi(ch);
	A = (long long int *)calloc(N, sizeof(long long int));
	for(i = 0; i < N; i += 1) {
		fgets(buf, sizeof(buf), stdin);
		ch = strtok(buf, " ");
		A[i] = atoll(ch);
	}

	qsort(A, N, sizeof(long long int), compare_llint_rise);

	count = 1;
	for(i = 1; i < N; i += 1) {
		if(A[i] == A[i - 1]) {
			count -= 1;
			A[i] = -1;
		} else {
			count += 1;
		}
	}

	printf("%d\n", count);

	free(A);
	return 0;
}
