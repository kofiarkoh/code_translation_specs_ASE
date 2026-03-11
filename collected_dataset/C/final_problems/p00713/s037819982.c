#include <stdio.h>
#include <math.h>

#define EPS 1e-5

float point[300][2] = { {0.0} };
float l[180000][2] = { { 0.0 } };

int search(int num);
int dist(int a, int b, double d);
int dist2(int a, int b, double d);

int main(void) {
	int i, num;

	while (1) {
		scanf("%d\n", &num);
		if (num == 0) {
			break;
		}

		for (i = 0; i < num; i++) {
			scanf("%f %f\n", &point[i][0], &point[i][1]);
		}

		printf("%d\n", search(num));

		for (i = 0; i < num; i++) {
			point[i][0] = 0.0;
			point[i][1] = 0.0;
		}
	}
	return 0;
}

int search(int num) {
	int i, j, s = 0;
	int count, maxcount;
	double a, a0, d;

	for (i = 0; i < num; i++) {
		for (j = 0; j < num; j++) {
			if (dist(i, j, 2) && i != j) {

				float X = (point[i][0] - point[j][0]);
				float Y = (point[i][1] - point[j][1]);

				d = sqrt(X*X + Y*Y);
				a = atan2(point[j][1] - point[i][1], point[j][0] - point[i][0]);
				a0 = acos(0.5 * d);

				l[s][0] = point[i][0] + cos(a0 + a);
				l[s][1] = point[i][1] + sin(a0 + a);
				s++;
				l[s][0] = point[i][0] + cos(a0 - a);
				l[s][1] = point[i][1] + sin(a0 - a);
				s++;
			}
		}
	}

	maxcount = 1;
	for (i = 0; i < s; i++) {
		count = 0;
		for (j = 0; j < num; j++) {
			if (dist2(j, i, 1)) {
				count++;
			}
		}
		if (count > maxcount) {
			maxcount = count;
		}
	}
	return maxcount;
}

int dist(int a, int b, double d){
	float X = (point[a][0] - point[b][0]);
	float Y = (point[a][1] - point[b][1]);
	if (sqrt(X*X + Y*Y) - d > EPS) {
		return (0);
	}
	return (1);
}

int dist2(int a, int b, double d) {
	float X = (point[a][0] - l[b][0]);
	float Y = (point[a][1] - l[b][1]);
	if (sqrt(X*X + Y*Y) - d > EPS) {
		return (0);
	}
	return (1);
}