#include <stdio.h>
#include <stdlib.h>

int main() {
	int n, *a, *b, i, j;
	scanf("%d", &n);
	a = malloc(n*sizeof(int));
	b = malloc(n*sizeof(int));
	for(i = 0; i < n; i++)
		scanf("%d", &a[i]);
	for(i = 0; i < n; i++)
		scanf("%d", &b[i]);
	for(i = 0; i < n; i++) {
		int t = a[i]^b[0], w = 1;
		for(j = 1; j < n && w; j++)
			if((a[(i + j)%n]^b[j]) != t)
				w = 0;
		if(w)
			printf("%d %d\n", i, t);
	}
	return 0;
}