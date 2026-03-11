#include <stdio.h>
#include <math.h>
int main() {
	int m, next_angle;
	double x = 0.0, y = 0.0;
	double prev_angle = M_PI_2;
	while (1) {
		scanf(" %d,%d", &m, &next_angle);
		if (m == 0 && next_angle == 0)
			break;
		x -= m * cos(prev_angle);
		y += m * sin(prev_angle);
		prev_angle += (double)next_angle * M_PI / 180;
	}
	printf("%d\n%d\n", (int)x, (int)y);
	return 0;
}