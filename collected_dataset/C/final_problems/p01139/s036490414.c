/* 
   AOJ 2014
   Surrounding Area
   2015/9/18
*/

#include<stdio.h>
#define W_MAX 51
#define H_MAX 51
#define DIR_N 4

int w, h, white_area = 0, black_area = 0, count, flag;
char area[H_MAX][W_MAX];
int dir[DIR_N][2] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
/* dir := down->right->up->left */

int is_area(int x, int y)
{
  return (0 <= x && x < w && 0 <= y && y < h && area[y][x] != 'V');
}

void dfs(int x, int y)
{
  int i, j, dir_x, dir_y;
  count++;
  area[y][x] = 'V';

  for(i = 0; i < DIR_N; i++)
    {
      dir_x = x + dir[i][0];
      dir_y = y + dir[i][1];
      if(is_area(dir_x, dir_y))
	{
	  if(area[dir_y][dir_x] == 'W')
	    {
	      if(flag == 0)
		{
		  flag = 1;
		}
	      if(flag == 2)
		{
		  flag = 3;
		}
	      continue;
	    }
	  if(area[dir_y][dir_x] == 'B')
	    {
	      if(flag == 0)
		{
		  flag = 2;
		}
	      if(flag == 1)
		{
		  flag = 3;
		}
	      continue;
	    }
	  dfs(dir_x, dir_y);
	}
    }
}

void ans_f(int x, int y)
{
  count = 0;
  flag = 0;
  dfs(x, y);
  if(flag == 1)
    {
      white_area += count;
    }
  if(flag == 2)
    {
      black_area += count;
    }
}

int main(void)
{
  int i, j;

  while(1)
    {
      white_area = 0;
      black_area = 0;
      count = 0;
      scanf("%d %d", &w, &h);
      if(w == 0 && h == 0)
	{
	  break;
	}
      for(i = 0; i < h; i++)
	{
	  scanf("%s", area[i]);
	}
      for(i = 0; i < h; i++)
	{
	  for(j = 0; j < w; j++)
	    {
	      if(area[i][j] == '.')
		{
		  ans_f(j, i);
		}
	    }
	}
      printf("%d %d\n",black_area, white_area);
    }

  return 0;
}