#include <stdio.h>
#include <stdlib.h>
#include <math.h>

double map[100][3];
double map_size;
double center[3];
double prev_max;
double down_rate;

#define fors(i, n) for (int i = 0; i < n; i++)

double update(double error_limit)
{
	double max = -1;
	double temp_1;
	int max_index;
	for (int i = 0; i < map_size; i++)
	{
		temp_1 = 0;
		temp_1 += pow(map[i][0] - center[0], 2);
		temp_1 += pow(map[i][1] - center[1], 2);
		temp_1 += pow(map[i][2] - center[2], 2);
		if (temp_1 > max)
		{
			max = temp_1;
			max_index = i;
		}
	}
	max = sqrt(max);
	if (down_rate < error_limit)
		return max;
	else
	{
		double vector[3];
		vector[0] = map[max_index][0] - center[0];
		vector[1] = map[max_index][1] - center[1];
		vector[2] = map[max_index][2] - center[2];
		center[0] += vector[0] * down_rate;
		center[1] += vector[1] * down_rate;
		center[2] += vector[2] * down_rate;
		prev_max = max;
		down_rate *= 0.99;
		return -1;
	}
}

int main()
{
	int count;
	double x, y, z;
	while (scanf("%d", &count) && count != 0)
	{
		center[0] = 0;
		center[1] = 0;
		center[2] = 0;
		fors(i, count)
		{
			scanf("%lf %lf %lf", &x, &y, &z);
			map[i][0] = x;
			map[i][1] = y;
			map[i][2] = z;
			center[0] += x;
			center[1] += y;
			center[2] += z;
		}
		map_size = count;
		center[0] /= map_size;
		center[1] /= map_size;
		center[2] /= map_size;
		prev_max = 10;
		down_rate = 1;
		while (update(1e-10) < 0)
			;
		printf("%.5f\n", update(1e-10));
	}
}
