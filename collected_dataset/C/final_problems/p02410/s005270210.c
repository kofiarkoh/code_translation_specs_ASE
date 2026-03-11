#include <stdio.h>
#include <stdlib.h>

int main() {
	int n, m;
	scanf("%d%d", &n, &m);
	int **a = (int**)malloc(sizeof(int*) * n);
	for (int i = 0; i < n; i++) {
		a[i] = (int*)malloc(sizeof(int) * m);
	}
	int* b = (int*)malloc(sizeof(int) * m);
	int* c = (int*)malloc(sizeof(int) * m);

	for (int i = 0; i < n; i++) {
		for (int j = 0; j < m; j++) {
			scanf("%d", &a[i][j]);
		}
	}

	
	for (int i = 0; i < m; i++) {
		scanf("%d", &b[i]);
	}

	for (int i = 0; i < n; i++) {
			c[i] = 0;
	}
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < m; j++) {
			c[i] = c[i] + a[i][j] * b[j];
		}
	}
	for (int i = 0; i < n; i++) {
		printf("%d\n", c[i]);
	}
	

}

