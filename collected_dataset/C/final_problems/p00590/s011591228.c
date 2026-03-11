#include<stdio.h>

int A[1500];

void maker() {
	int i, j, k;
	for (i = 1,k=0; i <= 10000; i++) {
		for (j = 2; j < i/2+1; j++) {
			if (i%j == 0)  break;
		}
		if (j >= (i/2+1)) A[k] = i,k++;
	}
}

int judge(int a) {
	int i;
	int j = 0;
	for (i = 1; i < 1231; i++) {
		if (a == A[i]) { j = 1; break; }
		if (a < A[i]) { break; }
	}
	return j;
}


int main() {
	int i,j,k;
	int n;
	int res;
	maker();

	while (scanf("%d", &n) != EOF) {
		res = 0;
		for (i = 1, j = n; i <= n; i++, j--) {
			if (judge(i) && judge(j)) res++;
		}
		printf("%d\n", res);
	}
	return 0;
}
