#include <stdio.h>          // printf(), scanf()
#include <stdbool.h>

const int INF = 100000000;

int
main(int argc, char **argv)
{
	const int c[8] = { 4, 1, 4, 1, 2, 1, 2, 1};
	int i, j;

	while (true)
	{
		int p[8];
		for (i = 0; i < 8; ++i)
		{
			if (scanf("%d", &p[i]) == EOF)
				goto l_end;
		}

		int min = INF;
		int vmin = INF;
		for (i = 0; i < 8; ++i)
		{
			int t = 0;
			for (j = 0; j < 8; ++j)
			{
				if (p[j] > c[(j + i) % 8])
					t += p[j] - c[(j + i) % 8];
			}

			if (t <= min)
			{
				int v = 0;
				for (j = 0; j < 8; ++j)
					v = 10 * v + c[(j + i) % 8];

				if (t < min || v < vmin)
				{
					min = t;
					vmin = v;
				}
			}
		}

		int ans[8];
		for (i = 7; i >= 0; --i)
		{
			ans[i] = vmin % 10;
			vmin /= 10;
		}

		for (i = 0; i < 8; ++i)
			printf("%d%c", ans[i], i < 7 ? ' ' : '\n');
	}

l_end:
	return 0;
}