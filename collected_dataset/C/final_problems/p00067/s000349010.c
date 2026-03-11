#include <stdio.h>

char map[16][16] = {0};

void delete_island(int x, int y)
{
	if (map[x][y] == 0) return;
	map[x][y] = 0;
	delete_island(x - 1, y);
	delete_island(x + 1, y);
	delete_island(x, y - 1);
	delete_island(x, y + 1);
}

int main(void)
{
	while (scanf("%s", &map[0][1]) != EOF){
		int i, j;
		int count = 0;
		
		for (i = 1; i < 12; i++) scanf("%s", &map[i][1]);
		for (i = 0; i < 12; i++){
			for (j = 0; j < 12; j++) map[i][j + 1] -= '0';
		}
		
		for (i = 0; i < 12; i++){
			for (j = 0; j < 12; j++){
				if (map[i][j + 1] == 1){
					count++;
					delete_island(i, j + 1);
				}
			}
		}
		printf("%d\n", count);
		scanf("%*c");
	}
	
	return 0;
}