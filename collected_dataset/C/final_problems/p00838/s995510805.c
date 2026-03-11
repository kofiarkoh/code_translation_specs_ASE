#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define fors(i, n) for (int i = 0; i < n; i++)

char maps[4][6][50];
int cases[24][6] = {{1, 2, 0, 5, 3, 4}, {1, 5, 2, 3, 0, 4}, {1, 3, 5, 0, 2, 4}, {1, 0, 3, 2, 5, 4}, {2, 1, 5, 0, 4, 3}, {2, 0, 1, 4, 5, 3}, {2, 4, 0, 5, 1, 3}, {2, 5, 4, 1, 0, 3}, {3, 4, 5, 0, 1, 2}, {3, 5, 1, 4, 0, 2}, {3, 1, 0, 5, 4, 2}, {3, 0, 4, 1, 5, 2}, {4, 0, 2, 3, 5, 1}, {4, 2, 5, 0, 3, 1}, {4, 5, 3, 2, 0, 1}, {4, 3, 0, 5, 2, 1}, {5, 2, 1, 4, 3, 0}, {5, 4, 2, 3, 1, 0}, {5, 1, 3, 2, 4, 0}, {5, 3, 4, 1, 2, 0}, {0, 2, 4, 1, 3, 5}, {0, 1, 2, 3, 4, 5}, {0, 3, 1, 4, 2, 5}, {0, 4, 3, 2, 1, 5}};

int min = 100000, n, data[4];

int color(char ch[5][50]) // input is rotated dices same face
{
	int counts[5] = {0};
	fors(i, n)
	{
		fors(j, n)
		{
			if (!strcmp(ch[i], ch[j])) // same color
				counts[i]++;
		}
	}
	int max = 0;
	fors(c, n)
	{
		if (counts[c] > counts[max]) // best color
			max = c;
	}
	int flag = 0;
	fors(i, n) if (strcmp(ch[i], ch[max])) // diff color count
		flag++;
	return flag;// return repaint count
}

void min_repaint()
{
	int sum = 0;
	fors(i, 6)
	{
		char temp[5][50]; //dices's same face
		fors(j, n)
		{
			strcpy(temp[j], maps[j][cases[data[j]][i]]); //set surface name, rotate by data
		}
		sum += color(temp);
	}
	if (sum < min)
		min = sum;
}

void set_data_dfs(int depth)
{
	if (depth == n - 1)
		min_repaint();
	else
		fors(i, 24)
		{
			data[depth] = i;
			set_data_dfs(depth + 1);
		}
}

int main()
{
	while (scanf("%d", &n) && n != 0)
	{
		fors(i, n) fors(j, 6)
			scanf("%s", maps[i][j]);
		min = 1000000;
		set_data_dfs(0);
		printf("%d\n", min);
	}
	return 0;
}
