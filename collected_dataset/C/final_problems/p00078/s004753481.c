#include <stdio.h>

int main(void)
{
	int n;
	while (scanf("%d", &n), n){
		int magic[16][16] = {0};
		int hn = n / 2;
		int x, y;
		int i, j;
		int number = 1;
		
		y = x = hn;
		y++;
		
		while (number <= n * n){
			if (magic[y][x]){
				y = (y + 1 + n) % n;
				x = (x - 1 + n) % n;
				continue;
			}
			magic[y][x] = number;
			y = (y + 1) % n;
			x = (x + 1) % n;
			number++;
		}
		for (i = 0; i < n; i++){
			for (j = 0; j < n; j++){
				printf("%4d", magic[i][j]);
			}
			puts("");
		}
	}
	return (0);
}