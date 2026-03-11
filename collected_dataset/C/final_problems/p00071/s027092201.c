#include <stdio.h>

char map[8][9];

void dfs(int tx, int ty)
{
	int i, j;
	
	if (0 > ty || ty >= 8 || 0 > tx || tx >= 8 || map[ty][tx] == '0') return;
	
	map[ty][tx] = '0';
	
	for (i = -3; i <= 3; i++){
		for (j = -3; j <= 3; j++){
			if ((i == 0 || j == 0) && i + j){
				dfs(tx + i, ty + j);
			}
		}
	}
	return;
}

int main(void)
{
	int n, t;
	int i;
	int sx, sy;
	
	scanf("%d", &n);
	
	t = 0;
	while (n--){
		for (i = 0; i < 8; i++){
			scanf("%s", map[i]);
		}
		
		scanf("%d %d", &sx, &sy);
		
		dfs(--sx, --sy);
		
		printf("Data %d:\n", ++t);
		for (i = 0; i < 8; i++){
			printf("%s\n", map[i]);
		}
	}
	
	return (0);
}