#include <stdio.h>

int main(void)
{
	int m, n, i, j, k, c;
	
	while (scanf("%d", &m), m != 0){
		scanf("%d", &n);
		c = 0;
		for (i = 1; i < 1000000; i *= 2){
			for (j = 1; i * j < 1000000; j *= 3){
				for (k = 1; i * j * k < 1000000; k *= 5){
					if (m <= i * j * k && i * j * k <= n){
						c++;
					}
				}
			}
		}
		printf("%d\n", c);
	}
	return (0);
}