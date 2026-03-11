#include <stdio.h>

int main(void)
{
	int n;
	
	while (scanf("%d", &n) * n != 0){
		int sum = 0;
		
		n /= 4;
		while (n-- > 0){
			int t;
			
			scanf("%d", &t);
			sum += t;
		}
		printf("%d\n", sum);
	}
	
	return 0;
}