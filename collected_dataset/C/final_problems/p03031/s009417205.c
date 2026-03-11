#include <stdio.h>

int main(void) {
	int N, M;
	int base = 1;
	int i, j, l;

	scanf("%d %d", &N, &M);

	for (i = 0;i < N;i++) {
		base *= 2;
	}


	int k[M - 1];
	int a[M - 1][M - 1];
	int b[M - 1][M - 1];
	int s[M - 1];;
	int p[M];

	for (i = 0;i < M;i++) {

		s[i] = 0;
		scanf("%d", &k[i]);
		if (k[i] > N) {
			return 0;
		}
		for (j = 0;j < k[i];j++) {

			scanf("%d", &a[i][j]);
			if (a[i][j] > N) {
				return 0;
			}
			b[i][j] = 1;
			for (l = 0;l < a[i][j];l++) {
				b[i][j] *= 2;
			}

			s[i] += b[i][j];
		};

		s[i] /= 2;
		
	}

	for (i = 0;i < M;i++) {

		scanf("%d", &p[i]);
	}

	int count = 0;
	int ssub[M - 1];

	for (j = 0;j < base;j++) {

		for (i = 0;i < M;) {
			int scount = 0;

			ssub[i] = s[i] & j;
						
			for (l = 0;l < N;l++) {
				scount += ssub[i] % 2;
				ssub[i] /= 2;
			}
		
			scount %= 2;

			if (scount == p[i]) {
				i++;
			}
			else { break; }
		}

		if (i == M) {
			count++;
		}
		else {
			count += 0;
		}
	}

	printf("%d", count);


	return 0;
}