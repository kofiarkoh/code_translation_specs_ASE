// AOJ 2097: Triangles
// 2017.11.7 bal4u@uu

#include <stdio.h>
#include <math.h>

#define PI    3.1415926535897932384626433832795
#define ROOT3 1.7320508075688772935274463415059

int main()
{
	int n;

	while (scanf("%d", &n) && n > 0) {
		if (n % 3 == 0) n /= 3;
		printf("%lf\n", n/(1/tan(PI/(3*n))+ROOT3));
	}
	return 0;
}