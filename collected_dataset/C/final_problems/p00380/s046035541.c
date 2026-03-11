#include<stdio.h>
void swap(int*, int, int);
int main(void) {
	static int n, a[300000], q, x[300000], y[300000];
	int flag = 0, cnt = 0;
	int stopi, stopj;
	int i, j, k;

	stopi = 0;
	stopj = 1;

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", &a[i]);
	}
	scanf("%d", &q);
	for (i = 0; i < q; i++) {
		scanf("%d %d", &x[i], &y[i]);
	}

	for (i = 0; i < q; i++) {
		
		while (a[stopi] <= a[stopj]) {
			stopi++;
			stopj++;
			if (stopi == n - 1) {
				flag = 1;
			}
		}
		if (flag) {
			break;
		}
		cnt++;
		swap(a, x[i], y[i]);
		//printf("stopi==%d stopj==%d\n", stopi, stopj);
	}
	//printf("stopi==%d stopj==%d\n", stopi, stopj);
	if (i == q) {
		printf("-1\n");
	}
	else {
		printf("%d\n", i);
	}
	
	return 0;
}
void swap(int *data, int x, int y) {
	int temp;
	temp = data[x - 1];
	data[x - 1] = data[y - 1];
	data[y - 1] = temp;
}
