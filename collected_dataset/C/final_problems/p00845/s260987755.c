#include <stdio.h>
#include <stdlib.h>
#include <math.h>

double star_coor[500][4];
int star_num;
double telescope_direction[50][4];
int telescope_num;

#define fors(i, n) for (int i = 0; i < n; i++)

int parse()
{
	scanf("%d", &star_num);
	if (star_num == 0)
		return 0;
	fors(i, star_num)
	{
		scanf("%lf %lf %lf", &star_coor[i][0], &star_coor[i][1], &star_coor[i][2]);
		star_coor[i][3] = 0;
	}
	scanf("%d", &telescope_num);
	fors(i, telescope_num)
		scanf("%lf %lf %lf %lf", &telescope_direction[i][0], &telescope_direction[i][1], &telescope_direction[i][2], &telescope_direction[i][3]);
	return 1;
}

int run()
{
	fors(i, star_num)
	{
		fors(j, telescope_num)
		{
			double edge1 = pow(telescope_direction[j][0] - star_coor[i][0], 2);
			edge1 += pow(telescope_direction[j][1] - star_coor[i][1], 2);
			edge1 += pow(telescope_direction[j][2] - star_coor[i][2], 2);
			double edge2 = pow(telescope_direction[j][0], 2);
			edge2 += pow(telescope_direction[j][1], 2);
			edge2 += pow(telescope_direction[j][2], 2);
			double edge3 = pow(star_coor[i][0], 2);
			edge3 += pow(star_coor[i][1], 2);
			edge3 += pow(star_coor[i][2], 2);
			double cosA = (edge2 + edge3 - edge1) / (2 * sqrt(edge2 * edge3));
			if (cosA > cos(telescope_direction[j][3]))
			{
				star_coor[i][3] = 1;
				break;
			}
		}
	}
	int res = 0;
	fors(i, star_num)
	{
		res += star_coor[i][3];
	}
	return res;
}

int main()
{
	while (parse())
	{
		printf("%d\n", run());
	}
}
