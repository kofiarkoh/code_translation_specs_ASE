#include<stdio.h>
#include<math.h>
#include<stdlib.h>
int main () {
	int n, i, j, max;
	int *a;
	// int a[200000] = {0};
	scanf("%d", &n);
	a = (int *)malloc(n * sizeof(int));
	for(i = 0; i < n; ++i) scanf("%d", &a[i]);
	for(i = 0; i < n; ++i){
		max = 0;
		for(j = 0; j < n; ++j){
			if(i == j){
				continue;
			}
			if(max < a[j]){
				max = a[j];
			}
		}
		printf("%d\n", max);
	}
	return 0;
}