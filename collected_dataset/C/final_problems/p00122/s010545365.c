#include <stdio.h>          // printf(), scanf()
#include <stdbool.h>

#define MAX_N 10
#define MAX_M 10
#define MAX_G 9

int sx, sy;
int n;
int scx[MAX_N], scy[MAX_N];

bool
b_survival()
{
	bool field[MAX_M][MAX_M];
	int px[MAX_G], py[MAX_G];
	static const int dx2[12] = {-1, 0, 1, 2, 2, 2,-1, 0, 1,-2,-2,-2};
	static const int dy2[12] = {-2,-2,-2,-1, 0, 1, 2, 2, 2,-1, 0, 1};

	int ix = 0;
	px[ix] = sx, py[ix] = sy;
	ix++;

	for (int u = 0; u < n; ++u)
	{
		for (int dy = -1; dy <= 1; ++dy)
		{
			for (int dx = -1; dx <= 1; ++dx)
			{
				int nx = scx[u] + dx, ny = scy[u] + dy;
				if (0 <= nx && nx < MAX_M && 0 <= ny && ny < MAX_M)
					field[ny][nx] = false;
			}
		}

		for (int i = 0; i < ix; ++i)
		{
			for (int j = 0; j < 12; ++j)
			{
				int nx = px[i] + dx2[j], ny = py[i] + dy2[j];
				if (0 <= nx && nx < MAX_M && 0 <= ny && ny < MAX_M)
					field[ny][nx] = true;
			}
		}

		ix = 0;
		for (int dy = -1; dy <= 1; ++dy)
		{
			for (int dx = -1; dx <= 1; ++dx)
			{
				int nx = scx[u] + dx, ny = scy[u] + dy;
				if (0 <= nx && nx < MAX_M && 0 <= ny && ny < MAX_M && field[ny][nx])
				{
					px[ix] = nx;
					py[ix] = ny;
					ix++;
				}
			}
		}

		if (ix == 0)
			return false;
	}

	return true;
}

int
main(int argc, char **argv)
{
	int lineno = 0;
	while (true)
	{
		lineno++;
		scanf("%d%d", &sx, &sy);
		if (sx == 0 && sy == 0)
			break;

		lineno++;
		scanf("%d", &n);

		lineno++;
		for (int i = 0; i < n; ++i)
			scanf("%d%d", &scx[i], &scy[i]);

		printf("%s\n", b_survival() ? "OK" : "NA");
	}

	return 0;
}