#include <stdio.h>

int mode[100];

int main(void)
{
	
	int num, n, i;
	
	while (scanf("%d", &num) != EOF){
		
		mode[num]++;
	}
	for (i = 0; i < 100; i++){
		num = 0;
		for (n = 0; n < 100; n++){
			
			if (mode[i] >= mode[n]){
				num++;
			}
			if (num == 100){
				printf("%d\n", i);
			}
		}
	}
	return (0);
}
				