#include <stdio.h>
#include <math.h>

double get_radian(double x1, double y1, double x2, double y2) {
	if(fabs(x1-x2) < 0.00000001) {
		if(y1 >= y2) {
			return 3 * M_PI / 2;
		}
		return M_PI / 2;
	} else if(fabs(y1-y2) < 0.00000001) {
		if(x1 >= x2) {
			return M_PI;
		}
		return 0.0;
	}
	
	if(x2 >= x1) {
		if(y2 >= y1) {
			return atan((y2-y1)/(x2-x1));
		}
		return 2 * M_PI + atan((y2-y1)/(x2-x1));
	} else {
		if(y2 >= y1) {
			return M_PI - atan((y2-y1)/(x1-x2));
		}
		return M_PI + atan((y2-y1)/(x2-x1));
	}
}

int main(void) {
	double base_x, base_y, x, y;
	double base_rad, target_rad;
	double rad;
	double r;
	int i, n;
	
	scanf("%lf %lf %lf %lf\n", &base_x, &base_y, &x, &y);
	scanf("%d\n", &n);
	base_rad = get_radian(base_x, base_y, x, y);
	
	for(i=0;i<n;i++) {
		scanf("%lf %lf\n", &x, &y);
		target_rad = get_radian(base_x, base_y, x, y);
		r = sqrt((x-base_x)*(x-base_x)+(y-base_y)*(y-base_y));
		
		// 後はどっち側に振れるか
		rad = fmod(2*M_PI+target_rad-base_rad, 2*M_PI);
		
		if(rad > M_PI) {
			rad = 2*M_PI - rad;
		}
		
		x = x + (2*r*sin(rad)) * cos(fmod(base_rad+3*M_PI/2, 2*M_PI));
		y = y + (2*r*sin(rad)) * sin(fmod(base_rad+3*M_PI/2, 2*M_PI));
		
		if(fabs(x) < 0.00000001) x = 0.0;
		if(fabs(y) < 0.00000001) y = 0.0;
		
		printf("%.10lf %.10lf\n", x, y);
	}
	
	return 0;
}