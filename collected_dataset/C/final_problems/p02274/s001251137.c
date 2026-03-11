#include <stdio.h>
#include <string.h>
//#include <limits.h>

#define MAX  500002
#define HALF 250002

int S[MAX] = {0};
int L[HALF] = {0};
int R[HALF] = {0};
long cnt = 0;

void Merge(int A[], int left, int mid, int right) {
	int  n1, n2;
	int i, j, k;
	memset(L, 0, sizeof(L));
	memset(L, 0, sizeof(L));
	
	n1 = mid - left;
	n2 = right - mid;
	
	for (i = 0; i < n1; ++i) {
		L[i] = A[left + i];
	}

	for (i = 0; i < n2; ++i) {
		R[i] = A[mid + i];
	}
	L[n1] = 1000000005;
	R[n2] = 1000000005;

	i = j = 0;
	for (k = left; k < right; ++k) {
		if (L[i] <= R[j]) {
			A[k] = L[i];
			++i;
		} else {
			A[k] = R[j];
			++j;
			cnt += (n1 -i);
		}
	}
	
	return;
}

void MergeSort(int A[], int left, int right) {
	int mid;
	if (left + 1 < right) {
		mid = (left + right) / 2;
		MergeSort(A, left, mid);
		MergeSort(A, mid, right);
		Merge(A, left, mid, right);
	}
	return;
}

int main(void) {
	int n, i;

	scanf("%d", &n);

	for (i = 0; i < n; ++i) {
		scanf("%d", &S[i]);
	}
	MergeSort(S, 0, n);
	printf("%ld\n", cnt);
	
	return 0;
}