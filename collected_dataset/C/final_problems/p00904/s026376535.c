#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int sq_map[150][150];
int sq_map_index = 0;

int main()
{
	for (sq_map_index = 0; sq_map_index < 150; sq_map_index++)
		for (int i = sq_map_index; i < 150; i++)
			sq_map[sq_map_index][i] = sq_map_index * sq_map_index + i * i;

	int input1, input2, counter;
	char output;
	for (scanf("%d", &counter); counter > 0; counter--)
	{
		scanf("%d %d", &input1, &input2);
		int x, y, m, n, p, q, temp;
		p = input1 * input1;
		q = input2 * input2;
		for (m = 0; m < 150; m++)
		{
			if (sq_map[m][m] > (p + q))
				break;
			for (n = m; n < 150; n++)
			{
				if (sq_map[m][n] > (p + q))
					break;
				if (m + n <= 1)
					continue;
				if ((p + q) % (sq_map[m][n]) == 0)
				{
					temp = (p + q) / (sq_map[m][n]);
					for (x = 0; x < 150; x++)
					{
						if (sq_map[x][x] > (p + q))
							break;
						for (y = x; y < 150; y++)
						{
							if (x + y <= 1)
								continue;
							if (sq_map[x][y] > (p + q))
								break;
							if (temp == (sq_map[x][y]))
							{
								goto C_END;
							}
						}
					}
				}
			}
		}
		printf("%c\n", 'P');
		continue;
	C_END:
		printf("%c\n", 'C');
	}
	return 0;
}
