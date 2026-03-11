#include<stdio.h>
#include<stdlib.h>

int main(void) {
	int A[20][6];
	int a[20];
	int L;
	int i,j;
	int cnt;
	int tmp;
	int ans;
	int sub;

	while (1) {
		for (i = 0; i < 20; i++) {
			for (j = 0; j < 6; j++) {
				A[i][j] = 0;
			}
		}
		scanf("%d", &a[0]);
		scanf("%d", &L);

		if (a[0] == 0 && L == 0) {
			break;
		}

		sub = a[0];
		for (i = 0; i < L; i++) {
			A[0][L - i-1] = sub % 10;
			sub = sub / 10;
		}
		/*
		for (i = 0; i < L; i++) {
			printf("%d ", A[0][i]);
		}
		*/

		cnt = 0;
		ans = 0;
		while (1) {
			int max[6] = {};
			int min[6] = {};
			int big = 0, small = 0;


			for (i = 0; i < L; i++) {
				max[i] = A[cnt][i];
				min[i] = A[cnt][i];
			}

			for (i = 0; i < L; ++i) {
				for (j = i + 1; j < L; ++j) {
					if (max[i] < max[j]) {
						tmp = max[i];
						max[i] = max[j];
						max[j] = tmp;
					}
				}
			}
			for (i = 0; i < L; ++i) {
				for (j = i + 1; j < L; ++j) {
					if (min[i] > min[j]) {
						tmp = min[i];
						min[i] = min[j];
						min[j] = tmp;
					}
				}
			}
			/*
			for (i = 0; i < L; i++) {
				printf("%d ", max[i]);
			}
			*/
			for (i = 0; i < L; i++) {
				big = big * 10 + max[i];
				small = small * 10 + min[i];
			}
			/*
			printf("\n%d %d\n", big,small);
			*/

			cnt++;
			a[cnt] = big - small;
			sub = a[cnt];
			for (i = 0; i < L; i++) {
				A[cnt][L - i-1] = sub % 10;
				sub = sub / 10;
			}
			/*
			for (i = 0; i < L; i++) {
				printf("%d ", A[cnt][i]);
			}
			*/
			for (i = 0; i < cnt; i++) {
				if (a[cnt] == a[i]) {
					printf("%d %d %d\n", i, a[cnt], cnt - i);
					ans = 1;
					break;
				}
			}
			if (ans == 1) {
				ans = 0;
				break;
			}
		}



	}
	return 0;
}
