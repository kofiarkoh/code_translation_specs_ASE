#include <stdio.h>

int main(void) {
	int n, m, l;
	int i, j, k;
	int a[100][100], b[100][100];
	
	scanf("%d %d %d", &n, &m, &l);
	
	for (i = 0; i < n; i++) {
		for (j = 0; j < m; j++) {
			scanf("%d", &a[i][j]);
		}
	}
	
	for (i = 0; i < m; i++) {
		for (j = 0; j < l; j++) {
			scanf("%d", &b[i][j]);
		}
	}
	
	for (i = 0; i < n; i++) {
		for (j = 0; j < l; j++) {
			int sum = 0;
			
			for (k = 0; k < m; k++) sum += a[i][k] * b[k][j];
			
			printf("%s%d", j ? " " : "", sum);
		}
		puts("");
	}
	
	return 0;
}