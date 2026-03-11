#include<stdio.h>
#include<math.h>
int main(void)
{
	double x, h,s;

	while (1) {
		scanf("%lf",&x);
		scanf("%lf", &h);
	    if ((x == 0) && (h == 0)) break;

		h = sqrt(h*h + ((x/2)*(x/2)));
		s = x*x + x*h*0.5 * 4;
		printf("%lf\n",s);
	}
	return 0;
}