/* AOJ 1144
   Curling 2.0
*/

#include<stdio.h>
#include<stdlib.h>

#define WH_MAX 20
int w, h;

typedef struct state
{
  int c;
  int x, y;
  int f[WH_MAX][WH_MAX];
}state;

typedef struct node
{
  state s;
  struct node *next;
}node;

struct node *head = NULL;
struct node *tail = NULL;



int empty()
{
  return head == NULL;
}

void enqueue(state s)
{
  node *new = malloc(sizeof(node));
  new->s = s;
  new->next = NULL;
  if(head == NULL)
    head = new;
  else
    tail->next = new;

  tail = new;
}

state dequeue()
{
  state ret = head->s;
  node *d = head;
  head = head->next;
  free(d);
  return ret;
}

int in_field(state s)
{
  return 0 <= s.x && s.x < w && 0 <= s.y && s.y < h;
}

int throw(state *s, int dir)
{
  int i;
  switch(dir)
    {
    case 0:
      {
	s->x++;
	if(in_field(*s) && s->f[s->y][s->x] != 1)
	  {
	    for(; ; s->x++)
	      {
		if(!in_field(*s))
		  return 0;
		else if(s->f[s->y][s->x] == 1)
		  {
		    s->f[s->y][s->x] = 0;
		    s->x--;
		    s->c++;
		    return 1;
		  }
		else if(s->f[s->y][s->x] == 3)
		  {
		    s->c++;
		    return 1;
		  }
	      }
	  }
	else
	  return 0;
      }
    case 1:
      {
	s->y++;
	if(in_field(*s) && s->f[s->y][s->x] != 1)
	  {
	    for(; ; s->y++)
	      {
		if(!in_field(*s))
		  return 0;
		else if(s->f[s->y][s->x] == 1)
		  {
		    s->f[s->y][s->x] = 0;
		    s->y--;
		    s->c++;
		    return 1;
		  }
		else if(s->f[s->y][s->x] == 3)
		  {
		    s->c++;
		    return 1;
		  }
	      }
	  }
	else
	  return 0;
      }
    case 2:
      {
	s->x--;
	if(in_field(*s) && s->f[s->y][s->x] != 1)
	  {
	    for(; ; s->x--)
	      {
		if(!in_field(*s))
		  return 0;
		else if(s->f[s->y][s->x] == 1)
		  {
		    s->f[s->y][s->x] = 0;
		    s->x++;
		    s->c++;
		    return 1;
		  }
		else if(s->f[s->y][s->x] == 3)
		  {
		    s->c++;
		    return 1;
		  }
	      }
	  }
	else
	  return 0;
      }
    case 3:
      {
	s->y--;
	if(in_field(*s) && s->f[s->y][s->x] != 1)
	  {
	    for(; ; s->y--)
	      {
		if(!in_field(*s))
		  return 0;
		else if(s->f[s->y][s->x] == 1)
		  {
		    s->f[s->y][s->x] = 0;
		    s->y++;
		    s->c++;
		    return 1;
		  }
		else if(s->f[s->y][s->x] == 3)
		  {
		    s->c++;
		    return 1;
		  }
	      }
	  }
	else
	  return 0;
      }
    }
}

int search(state s)
{
  int i;
  state tmp;
  enqueue(s);

  while(!empty())
    {
      state now = dequeue();
      if(now.c >= 11)
	{
	  return -1;
	}
      else if(now.f[now.y][now.x] == 3)
	{
	  return now.c;
	}
      else
	{
	  for(i = 0; i < 4; i++)
	    {
	      tmp = now;
	      if(throw(&tmp, i))
		enqueue(tmp);
	    }
	}
    }
  return -1;
}

int main(void)
{
  int i, j;
  state s;
  while(scanf("%d %d", &w, &h), (w && h))
    {
      head = NULL;
      tail = NULL;
      for(i = 0; i < h; i++)
	for(j = 0; j < w; j++)
	  scanf("%d", &s.f[i][j]);

      for(i = 0; i < h; i++)
	for(j = 0; j < w; j++)
	  if(s.f[i][j] == 2)
	    {
	      s.x = j;
	      s.y = i;
	      break;
	    }
      s.c = 0;
      printf("%d\n", search(s));
    }
  return 0;
}