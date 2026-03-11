#include <stdio.h>

int main(void)
{
	int n;
	
	while (scanf("%d", &n) != EOF){
		while (n > 0){
			n -= 39;
		}
		n += 39;
		if (n <= 9){
			printf("3C0%d\n", n);
		}
		else {
			printf("3C%d\n", n);
		}
	}
	
	return (0);
}