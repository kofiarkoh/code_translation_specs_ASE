#include <stdio.h>
#include <string.h>

int main(void)
{
	long long ans, num[16][16];
	int w, h;
	int i, j;
	int map[16][16];
	
	while (1){
		scanf("%d%d", &w, &h);
		
		if (w + h == 0){
			break;
		}
		
		for (i = 0; i < h; i++){
			for (j = 0; j < w; j++){
				scanf("%d", &map[i][j]);
			}
		}
		
		memset(num, 0, sizeof(num));
		for (i = 0; i < w; i++){
			if (map[0][i] != 1){
				num[0][i] = 1;
			}
		}
		for (i = 1; i <= h; i++){
			for (j = 0; j < w; j++){
				if (j > 0 && map[i - 1][j - 1] == 0 && map[i][j] != 2){
					num[i][j] += num[i - 1][j - 1];
				}
				if (map[i - 1][j] == 0){
					num[i][j] += num[i - 1][j];
				}
				if (j < w - 1 && map[i - 1][j + 1] == 0 && map[i][j] != 2){
					num[i][j] += num[i - 1][j + 1];
				}
				if (i > 1 && map[i - 2][j] == 2){
					num[i][j] += num[i - 2][j];
				}
			}
		}
		ans = 0;
		for (j = 0; j < w; j++){
			ans += num[h - 1][j];
		}
		
		printf("%lld\n", ans);
	}
	
	return (0);
}