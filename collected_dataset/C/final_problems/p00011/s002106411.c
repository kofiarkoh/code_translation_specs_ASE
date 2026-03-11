#include <stdio.h>

int main()
{
	int w, n, a, b, x, i, rs[30];

	scanf("%d %d", &w, &n);

	for(i = 0; i < w; i++){
		rs[i] = i+1;
	}

	for(i = 0; i < n; i++){
		scanf("%d,%d", &a, &b);

		x = rs[a-1];
		rs[a-1] = rs[b-1];
		rs[b-1] = x;
	}

	for(i = 0; i < w; i++){
		printf("%d\n", rs[i]);
	}
	
	return 0;
}