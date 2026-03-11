/* 
   AOJ 1165
   Pablo Squareson's Headache
   2015.4.29
 */

#include<stdio.h>

struct block
{
  int x, y;
};


int width = 1, height = 1;

void calc_width_height(int n)
{
  int i, target_block, direction, max_x = 0, max_y = 0, min_x = 0, min_y = 0;
  struct block block_array[200];

  block_array[0].x = 0;
  block_array[0].y = 0;

  for(i = 1; i < n; i++)
    {
      scanf("%d %d", &target_block, &direction);
      switch(direction)
	{
	case 0:
	  block_array[i].x = block_array[target_block].x - 1;
	  block_array[i].y = block_array[target_block].y;
	  break;
	case 1:
	  block_array[i].x = block_array[target_block].x;
	  block_array[i].y = block_array[target_block].y - 1;
	  break;
	case 2:
	  block_array[i].x = block_array[target_block].x + 1;
	  block_array[i].y = block_array[target_block].y;
	  break;
	case 3:
	  block_array[i].x = block_array[target_block].x;
	  block_array[i].y = block_array[target_block].y + 1;
	  break;
  	}   
    }
  for(i = 0; i < n; i++)
    {
      if(max_x < block_array[i].x)
	{
	  max_x = block_array[i].x;
	}
      if(max_y < block_array[i].y)
	{
	  max_y = block_array[i].y;
	}
      if(min_x > block_array[i].x)
	{
	  min_x = block_array[i].x;
	}
      if(min_y > block_array[i].y)
	{
	  min_y = block_array[i].y;
	}
    }

  /* for(i = 0; i < n; i++) */
  /*   { */
  /*     printf("[%d]: x = %d, y = %d\n", i, block_array[i].x, block_array[i].y); */
  /*   } */

  width += max_x - min_x;
  height += max_y - min_y;
  
  return ;
}

int main(void)
{
  int n; 
  while(1)
    {
      scanf("%d", &n);
      if(n == 0)
	{
	  break;
	}
      
      width = 1;
      height = 1;

      calc_width_height(n);
      printf("%d %d\n", width, height);
      
    }
  return 0;
}