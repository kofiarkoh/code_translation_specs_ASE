#include <stdio.h>
#include <stdlib.h>

int main()
{
	long long x, y;
	long long ans = 0;

	scanf("%lld %lld", &x, &y);

	if ((x < 0 && y < 0 && llabs(x) < llabs(y)) ||
			(x > 0 && y > 0 && llabs(x) > llabs(y))) {
		ans++;
		
		x = -x;
	}

	ans = ans + llabs( llabs(x) - llabs(y) );
	x = x + llabs( llabs(x) - llabs(y) );

	if (x != y) {
		ans++;
	}

	printf("%lld\n", ans);
	
	return 0;
}
