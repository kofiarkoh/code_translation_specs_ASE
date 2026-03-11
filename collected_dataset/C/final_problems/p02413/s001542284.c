#include <stdio.h>

int main(){
	int r, c, i, j;
	int h[100][100] = {0};
	
	scanf("%d %d", &r, &c );
	for(i = 0; i < r; i++){
		for(j = 0; j < c; j++){
			scanf("%d", &h[i][j]);
		}
	}
	
	for(i = 0; i < r; i++){
		for(j = 0; j < c; j++){
			h[i][c] += h[i][j];
		}
	}
	
	for(j = 0; j < c + 1; j++){
		for(i = 0; i < r; i++){
			h[r][j] += h[i][j];
		}
	}
	for(i = 0; i < r + 1; i++){
		for(j = 0; j < c + 1; j++){
			printf("%d ", h[i][j]);
		}
		printf("\n");
	}
	
	return 0;
}