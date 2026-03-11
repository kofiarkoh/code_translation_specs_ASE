#include <stdio.h>

int main(void)
{
	float m, n;
	int i, j;
	int num;
	int count;
	
	while (1){
		scanf("%f", &m);
		if (m ==  0){
			break;
		}
		scanf("%f", &n);				
		count = 0;
		for (i = m; i <= n; i++){
			num = i;
			for (j = 0; j < 100; j++){
				if (num / 2 != 0 && num % 2 == 0){
					num /= 2;
				}
				if (num / 3 != 0 && num % 3 == 0){	
					num /= 3;
				}
				if (num / 5 != 0 && num % 5 == 0){
					num /= 5;
				}
			}
			if (num == 1){
				count += 1;
			}
		}
		printf("%d\n", count);
	}
	
	return (0);
}