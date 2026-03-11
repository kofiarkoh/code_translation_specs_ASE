/* 
   AOJ 2150
   Matshzaki Number
   2015/7/23
*/

#include<stdio.h>
#include<stdlib.h>

#define MAX_PRIME 200000
#define MAX_P 100

void make_prime_list(int *prime_list)
{
  int i, j;

  for(i = 0; i < MAX_PRIME; i++)
    {
      prime_list[i] = 1;
    }

  prime_list[0] = 0;
  prime_list[1] = 0;

  for(i = 0; i < MAX_PRIME; i++)
    {
      if(prime_list[i] == 1)
	{
	  for(j = i; i * j < MAX_PRIME && i * j > 0; j++)
	    {
	      prime_list[i * j] = 0;
	    }
	}
    }
}

void make_next_prime_list(int* prime_list, int* next_prime_list)
{
  int i, next_prime = -1;

  for(i = MAX_PRIME - 1; i >= 0; i--)
    {
      if(prime_list[i] == 1)
	{
	  break;
	}
      next_prime_list[i] = -1;
    }

  for(; i >= 0; i--)
    {
      next_prime_list[i] = next_prime;

      if(prime_list[i] == 1)
	{
	  next_prime = i;
	}
    }
}

void get_next_prime_set(int n, int* prime_list, int* next_prime_set)
{
  int i, j = 0;
  
  for(i = n + 1; i < MAX_PRIME; i++)
    {
      if(prime_list[i] == 1)
	{
	  next_prime_set[j] = i;
	  j++;
	}
    }
}

void get_sum_list(int* next_prime_set, int* sum_list, int p)
{
  int i, j, k = 0;

  for(i = 0; i < p; i++)
    {
      for(j = i; j < p; j++)
	{
	  sum_list[k] = next_prime_set[i] + next_prime_set[j];
	  k++;
	}
    }
}

int from_one_to_p_sum(int p, int n)
{
  if(p >= 1)
    {
      return from_one_to_p_sum(p - 1, n + p);
    }
  return n;
}

int sort(const void * a, const void * b)
{
  if(*(int *)a < *(int *)b)
    {
      return -1;
    }
  else if(*(int *)a == *(int *)b)
    {
      return 0;
    }
  return 1;
}

int ans_f(int n, int p, int* prime_list)
{

  /* 
     --------next_prime_set is a prime list from the prime that is next to 'n' to MAX_PRIME

     next_prime_set[0] + next_prime_set[0], nps[0] + nps[1],....nps[0] + nps[p]
     nps[1] + nps[1], nps[1] + nps[2],...nps[1] + nps[p]
     .......
     nps[p-1] + nps[p-1], nps[p-1] + nps[p]
     nps[p] + nps[p]
     
     ---------------------------------------------------------------------------------------
  */


  int i, num_of_sum_list = 0, next_prime_set[MAX_PRIME], sum_list[MAX_PRIME];
  
  get_next_prime_set(n, prime_list, next_prime_set);

  /* for(i = 0; i < 20; i++) */
  /*   { */
  /*     printf("%3d", next_prime_set[i]); */
  /*   } */
  /* printf("\n"); */

  get_sum_list(next_prime_set, sum_list, p);

  /* for(i = 0; i < 20; i++) */
  /*   { */
  /*     printf("%3d", sum_list[i]); */
  /*   } */
  /* printf("\n"); */

  num_of_sum_list = from_one_to_p_sum(p, 0);

  qsort((void *) sum_list, num_of_sum_list, sizeof(int), sort);

  /* for(i = 0; i < 20; i++) */
  /*   { */
  /*     printf("%3d", sum_list[i]); */
  /*   } */
  /* printf("\n"); */
  
  return sum_list[p - 1];
}

int main(void)
{
  int n, p, ans, prime_list[MAX_PRIME], next_prime_list[MAX_PRIME], i;

  make_prime_list(prime_list);
  //make_next_prime_list(prime_list, next_prime_list);

  while(1)
    {
      scanf("%d %d", &n, &p);
      if(n == -1 && p == -1)
	{
	  break;
	}
      ans = ans_f(n, p, prime_list);
      printf("%d\n", ans);
    }
  return 0;
}