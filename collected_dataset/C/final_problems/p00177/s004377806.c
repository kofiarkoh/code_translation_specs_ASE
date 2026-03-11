#include <stdio.h>
#include <math.h>
#define PI 3.141592653589793
#define r 6378.1
int main(void) {
	while(1){
		double a,b,c,d;
		scanf("%lf %lf %lf %lf\n",&a,&b,&c,&d);
		if(a==(-1)&&b==(-1)&&c==(-1)&&d==(-1)) break;
		double a1,b1,c1,d1;
		a1=a*PI/180.0;
		b1=b*PI/180.0;
		c1=c*PI/180.0;
		d1=d*PI/180.0;
		double e1,f;
		e1=d1-b1;
		f=r*acos((sin(a1)*sin(c1))+(cos(a1)*cos(c1)*cos(e1)));
		printf("%.0f\n",f);
	}
	return 0;
}
