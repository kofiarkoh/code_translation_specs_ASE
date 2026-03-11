/* 
   AOJ 1149
   Cut the Cake

*/

#include<stdio.h>
#include<stdlib.h>

#define MAX_CAKE 102

typedef struct cake
{
  int w, d, area;
}cake;

int cake_num = 1;
cake cake_arr[MAX_CAKE];



int comp(const void *a, const void *b)
{
  cake cake_a = *(cake *)a;
  cake cake_b = *(cake *)b;

  if(cake_a.area < cake_b.area)
    return -1;
  else
    return 1;
}

void new(int w, int d)
{
  cake_arr[cake_num].w = w;
  cake_arr[cake_num].d = d;
  cake_arr[cake_num].area = w * d;
}

void update_cake_num(int p)
{
  int i;
  cake tmp;
  for(i = p + 1; i <= cake_num; i++)
    cake_arr[i - 1] = cake_arr[i];
}

void cut(cake cutted_cake, int s)
{
  int w = cutted_cake.w, d = cutted_cake.d;
  cake cake_0, cake_1;
  s = s % (2 * (w + d));
  if(s <= w)
    {
      cake_0.w = s, cake_0.d = d;
      cake_1.w = w - s, cake_1.d = d;
    }
  else if(s <= w + d)
    {
      cake_0.w = w, cake_0.d = s - w;
      cake_1.w = w, cake_1.d = w + d - s;
    }
  else if(s <= 2 * w + d)
    {
      cake_0.w = s - (w + d), cake_0.d = d;
      cake_1.w = 2 * w + d - s, cake_1.d = d;
    }
  else
    {
      cake_0.w = w, cake_0.d = s - (2 * w + d);
      cake_1.w = w, cake_1.d = 2 * (w + d) - s;
    }

  cake_0.area = cake_0.w * cake_0.d, cake_1.area = cake_1.w * cake_1.d;

  if(cake_0.area == 0)
    cake_arr[cake_num] = cake_1;
  else if(cake_1.area == 0)
    cake_arr[cake_num] = cake_0;
  else if(cake_0.area < cake_1.area)
    {
      cake_arr[cake_num++] = cake_0;
      cake_arr[cake_num] = cake_1;
    }
  else
    {
      cake_arr[cake_num++] = cake_1;
      cake_arr[cake_num] = cake_0;
    }
}

/* 
   --test-case--
   3 5 6
   1 18
   2 19
   1 2
   ------------
*/


void init(void)
{
  int i;
  for(i = 1; i <=cake_num; i++)
    cake_arr[i].w = 0, cake_arr[i].d = 0, cake_arr[i].area = 0;

  cake_num = 1;
}


int main(void)
{
  int n, w, d, i, p, s;

  while(scanf("%d %d %d", &n, &w, &d), (n + w + d))
    {

      new(w, d);
      for(i = 0; i < n; i++)
	{
	  scanf("%d %d", &p, &s);
	  cake cutted_cake = cake_arr[p];
	  update_cake_num(p);
	  cut(cutted_cake, s);
	}
      qsort((void *)(cake_arr + 1), n + 1, sizeof(cake), comp);
      for(i = 1; i <= cake_num; i++)
	{
	  if(i == cake_num)
	    printf("%d\n", cake_arr[i].area);
	  else
	    printf("%d ", cake_arr[i].area);	  
	}
      init();
    }
  return 0;
}