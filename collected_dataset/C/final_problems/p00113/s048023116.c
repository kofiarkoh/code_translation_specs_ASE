#include <stdio.h>

int main(void)
{
	int p, q, dig, i, j, f, quo[2048], sur[2048];
	int loop;
	
	while (scanf("%d %d", &p, &q) != EOF){
		for (i = 0; i < 2048; i++){
			quo[i] = 0;
			sur[i] = 0;
		}
		
		quo[0] = p;
		
		dig = 0;
		while (1){
			quo[dig + 1] = quo[dig] % q * 10;
			sur[dig] = quo[dig] % q;
			quo[dig] = quo[dig] / q;
			
			if (sur[dig] == 0){
				break;
			}
			
			for (j = 1, f = 0; j < dig; j++){
				if (sur[dig] == sur[j]){
					f = 1;
					loop = j;
					break;
				}
			}
			if (f == 1){
				break;
			}
			
			dig++;
		}
		
		for (i = 1; i <= dig; i++){
			printf("%d", quo[i]);
		}
		puts("");
		if (f == 1){
			for (i = 1; i <= dig; i++){
				if (i > loop){
					printf("^");
				}
				else {
					printf(" ");
				}
			}
			puts("");
		}
	}
	
	return (0);
}