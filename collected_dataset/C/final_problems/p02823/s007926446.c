#include <stdio.h>

int main()
{
	long n,a,b;
	long num;

	scanf("%ld", &n);
	scanf("%ld", &a);
	scanf("%ld", &b);

	
	if ((b-a)%2 ==0) {
		num = (b-a)/2;
	}
	else {
		if (n -b > a-1) {
			num = a - 1;
			num++;
			
			b -= num;
			a = 1;
			num += (b - a) / 2;
		}
		else {
			num = n-b;
			num++;

			a += num;
			b = n;
			num += (b - a) / 2;

		}

		
	}
	

	printf("%ld", num);
	
}