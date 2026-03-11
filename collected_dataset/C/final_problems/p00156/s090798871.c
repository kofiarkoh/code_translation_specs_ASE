#include <stdio.h>          // printf(), fprintf(), scanf()
#include <stdlib.h>         // exit()
#include <stdbool.h>

#define SZ_FIELD 100
#define MAX_M 10000
#define QUE_SIZE 10000

typedef struct que_tbl
{
	int x;
	int y;
} que_t;

int N, M;
char field[SZ_FIELD][SZ_FIELD + 1];
const int dx[4] = {-1, 0, 1, 0};
const int dy[4] = { 0,-1, 0, 1};
que_t queue[QUE_SIZE];
int mx[MAX_M];
int my[MAX_M];
int que_in;
int que_out;
int ix;

void
cleanup(int ecode)
{
	exit(ecode);
}

int
que_empty()
{
	return que_in == que_out;
}

void
que_push(int x, int y)
{
	queue[que_in].x = x;
	queue[que_in].y = y;
	que_in = (que_in + 1) % QUE_SIZE;

	if (que_in == que_out)
	{
		fprintf(stderr, "Queue overflow. size:%d\n", QUE_SIZE);
		cleanup(1);
	}
}

void
que_pop()
{
	if (que_out == que_in)
	{
		fprintf(stderr, "Queue underflow\n");
		cleanup(1);
	}

	que_out = (que_out + 1) % QUE_SIZE;
}

void
que_front(que_t *que)
{
	que->x = queue[que_out].x;
	que->y = queue[que_out].y;
}

void
dfs(int x, int y)
{
	field[y][x] = '.';
	for (int i = 0; i < 4; ++i)
	{
		int nx = x + dx[i], ny = y + dy[i];
		if (0 <= nx && nx < N && 0 <= ny && ny < M && field[ny][nx] == '#')
			dfs(nx, ny);
	}
}

bool
bfs()
{
	que_t que;

	ix = 0;
	while (!que_empty())
	{
		que_front(&que); que_pop();
		int x = que.x, y = que.y;
		if (x == 0 || x == N - 1 || y == 0 || y == M - 1)
			return true;

		for (int i = 0; i < 4; ++i)
		{
			int nx = x + dx[i], ny = y + dy[i];
			if (0 <= nx && nx < N && 0 <= ny && ny < M)
			{
				if (field[ny][nx] == '.')
				{
					field[ny][nx] = ' ';
					que_push(nx, ny);
				}
				else if (field[ny][nx] == '#')
				{
					mx[ix] = nx, my[ix] = ny;
					ix++;
				}
			}
		}
	}

	return false;
}

int
solve()
{
	int cx, cy;

	for (cy = 0; cy < M; ++cy)
	{
		for (cx = 0; cx < N; ++cx)
		{
			if (field[cy][cx] == '&')
				break;
		}

		if (cx < N)
			break;
	}

	que_in = 0;
	que_out = 0;

	field[cy][cx] = ' ';
	que_push(cx, cy);

	int count = 0;
	while (!bfs())
	{
		que_in = 0;
		que_out = 0;

		for (int i = 0; i < ix; ++i)
		{
			que_push(mx[i], my[i]);
			dfs(mx[i], my[i]);
		}

		count++;
	}

	return count;
}

int
main(int argc, char **argv)
{
	while (true)
	{
		scanf("%d%d", &N, &M);
		if (N == 0 && M == 0)
			break;

		for (int i = 0; i < M; ++i)
			scanf("%s", &field[i][0]);

		printf("%d\n", solve());
	}

	return 0;
}
