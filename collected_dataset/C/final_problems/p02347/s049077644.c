#include <stdio.h>

int main(void) {
	int i, j, n, q;
	int x1, x2, y1, y2;
	int x, y, c;
	int A[500005][2];
	
	scanf("%d\n", &n);
	for(i=0;i<n;i++) {
		scanf("%d %d\n", &A[i][0], &A[i][1]);
	}
	
	scanf("%d\n", &q);
	for(i=0;i<q;i++) {
		scanf("%d %d %d %d\n", &x1, &x2, &y1, &y2);
		for(j=0,c=0;j<n;j++) {
			if(c == 100) break;
			x = A[j][0];
			if(x1>x) continue;
			if(x>x2) continue;
			y = A[j][1];
			if(y1>y) continue;
			if(y>y2) continue;
			printf("%d\n", j);
			c++;
		}
		
		printf("\n");
	}
	
	return 0;
}