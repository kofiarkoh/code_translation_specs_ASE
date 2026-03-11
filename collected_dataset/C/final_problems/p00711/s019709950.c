#include<stdio.h>
#include<stdlib.h>

int width, height, cnt = 0, direc[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

void dfs(char **s, int x, int y)
{
  int i, next_x = 0, next_y = 0;
  cnt++;
  for(i = 0; i < 4; i++)
    {
      next_x = x + direc[i][0];
      next_y = y + direc[i][1];
      //      printf("%d %d\n", next_x, next_y);
      if(next_x >= 0 && next_x < width && next_y >= 0 && next_y < height && s[next_y][next_x] == '.')
	{
	  //      printf("hoge");
	  s[next_y][next_x] = '#';
	  dfs(s, next_x, next_y);
	}
    }
}

void search(char **s)
{
  int i, j;

  for(i = 0; i < height; i++)
    {
      for(j = 0; j < width; j++)
	{
	  if(s[i][j] == '@')
	    {
	      dfs(s, j, i);
	    }
	}
    }
}

int main(void)
{
  int x, y, i, j;
  char **s;

  while(1)
    {
      scanf("%d %d", &width, &height);
      if(width == 0 && height == 0)
	{
	  break;
	}
      s = malloc(sizeof(char *) * height);

      for(i = 0; i < height; i++)
	{
	  s[i] = malloc(sizeof(char) * (width + 1));
	  scanf("%s", s[i]);
	}
      
      search(s);
      printf("%d\n", cnt);
      cnt = 0;
      /* printf("\n"); */

      /* for(i = 0; i < y; i++) */
      /* 	{ */
      /* 	  printf("%s", s[i]); */
      /* 	  printf("\n"); */
      /* 	} */
    }
  return 0;
}