#include<stdio.h>
int main() {

	int n, k, a[200001], i, j = 0;


	scanf("%d%d", &n, &k);

	for (i = 0; i < n; i++)
		scanf("%d", &a[i]);

	for (i = k; i < n; i++) {
		if (a[j] < a[i])
			printf("Yes\n");
		else
			printf("No\n");
		j++;
	}
	
	return 0;
}