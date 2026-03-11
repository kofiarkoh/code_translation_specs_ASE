#include <stdio.h>

int done[4192][5][4][3];
int memo[4192][5][4][3];

int min(int g, int yen, int a, int b, int c)
{
	int A, B, C;
	int MIN;
	
	if (a == 5){
		yen -= 380 * 5 * 20 / 100;
		a = 0;
	}
	if (b == 4){
		yen -= 550 * 4 * 15 / 100;
		b = 0;
	}
	if (c == 3){
		yen -= 850 * 3 * 12 / 100;
		c = 0;
	}
	
	if (g == 0){
		return (yen);
	}
	if (g < 0){
		return (999999);
	}
	
	A = min(g - 200, yen + 380, a + 1, b, c);
	B = min(g - 300, yen + 550, a, b + 1, c);
	C = min(g - 500, yen + 850, a, b, c + 1);
	
	MIN = A;
	
	if (B < MIN){
		MIN = B;
	}
	
	if (C < MIN){
		MIN = C;
	}
	
	done[g][a][b][c] = 1;
	memo[g][a][b][c] = MIN;
	
	return (memo[g][a][b][c]);
}

int main(void)
{
	int g;
	int i, j, k, l;
	
	while (scanf("%d", &g), g != 0){
		for (i = 0; i < 4192; i++){
			for (j = 0; j < 5; j++){
				for (k = 0; k < 4; k++){
					for  (l = 0; l < 3; l++){
						done[i][j][k][l] = 0;
						memo[i][j][k][l] = 0;
					}
				}
			}
		}
		
		printf("%d\n", min(g, 0, 0, 0, 0));
	}
	
	return (0);
}