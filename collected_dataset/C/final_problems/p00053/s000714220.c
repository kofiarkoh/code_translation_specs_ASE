#include <stdio.h>
#include <math.h>

int main(void)
{
	char number[104730] = {0};
	int i, j, n;
	long sum;

	for (i = 3; i <= 104729; i += 2) number[i] = 1;
	number[2] = 1;
	for (i = 3; i * i <= 104729; i += 2){
		if (number[i] == 1){
			for (j = 2; i * j <= 104729; j++){
				number[i * j] = 0;
			}
		}
	}
	
	while (1){
		scanf("%d", &n);
		
		if (n == 0) break;
		
		sum = 0;
		j = 2;
		while (n != 0){
			if (number[j] == 1){
				sum += (long)j;
				n--;
			}
			j++;
		}
		
		printf("%ld\n", sum);
	}
	return (0);
}