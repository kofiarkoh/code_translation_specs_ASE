/* 
   AOJ 2001
   Amida, the Cty of Miracle

 */

#include<stdio.h>
#include<stdlib.h>

struct node
{
  int height, from, to;
  struct node *next;
};

struct node *head = NULL;


void traverse()
{
  struct node *next;

  for(next = head; next != NULL; next = next->next)
    {
      printf("%d %d %d ->", next->height, next->from, next->to);
    }
  printf("\n");
}


void make_amida(int height, int first_line, int second_line)
{
  struct node *line = malloc(sizeof(*line)), *next = head, *prev = head;
  
  if(head == NULL)
    {
      head = line;
      line->next = NULL;
    }
  else
    {
      while(next != NULL && next->height >= height)
	{
	  prev = next;
	  next = next->next;
	}
      if(next == head)
	{
	  head = line;
	  line->next = next;
	}
      else
	{

	  prev->next = line;
	  line->next = next;
	}
    }

  line->height = height;
  line->from = first_line;
  line->to = second_line;

  //traverse();
}

int search_goal_line(int target_line)
{
  struct node *next;
  int current_line = target_line;

  for(next = head; next != NULL; next = next->next)
    {
      if(next->from == current_line)
	{
	  current_line = next->to;
	}
      else if(next->to == current_line)
	{
	  current_line = next->from;
	}
    }
  return current_line;
}



int main(void)
{
  int i, total_vertical_line, total_horizontal_line, target_line, height, first_line, second_line, hoge;

  while(1)
    {
      scanf("%d %d %d", &total_vertical_line, &total_horizontal_line, &target_line);

      if(total_horizontal_line == 0 && total_vertical_line == 0 && target_line == 0)
	{
	  break;
	}

      for(i = 0; i < total_horizontal_line; i++)
	{

	  scanf("%d %d %d", &height, &first_line, &second_line);
	  make_amida(height, first_line, second_line);
	}
      
      printf("%d\n", search_goal_line(target_line));

      head = NULL;
    }

  return 0;
}