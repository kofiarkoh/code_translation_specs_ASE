#include <stdio.h>
int main() {
	int N[4], i;
	int check[4] = {0};
	scanf("%d %d %d %d", &N[0], &N[1], &N[2], &N[3]);	
	for (i = 0; i < 4; i++) {
		if(N[i] == 1) { check[0] += 1; }
		if(N[i] == 4) { check[1] += 1; }
		if(N[i] == 7) { check[2] += 1; }
		if(N[i] == 9) { check[3] += 1; }
	}
	for (i = 0; i < 4; i++) {
		if (check[i] != 1) {
			printf("NO\n");
			return 0;			
		}
	}
	printf("YES\n");
	return 0;
}