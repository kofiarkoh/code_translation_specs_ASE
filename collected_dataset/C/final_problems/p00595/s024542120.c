#include <stdio.h>

int main(void)
{
	long a, b, t;
	
	while (scanf("%ld %ld", &a, &b) != EOF){
		while (b != 0){
			if (a < b){
				t = a;
				a = b;
				b = t;
			}
			t = a % b;
			a = b;
			b = t;
		}
		printf("%ld\n", a);
		
	}
	return (0);
}