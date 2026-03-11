#include <stdio.h>

int main(void)
{
	int n;
	int i, j, k, l;
	int c;
	
	while (scanf("%d", &n) != EOF){
		c = 0;
		for (i = 0; i <= n; i++){
			for (j = 0; j <= n; j++){
				for (k = 0; k <= n; k++){
					for (l = 0; l <= n; l++){
						if (i + j + k + l == n){
							c++;
						}
					}
				}
			}
		}
		printf("%d\n", c);
	}
	
	return (0);
}