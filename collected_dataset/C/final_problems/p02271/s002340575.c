#include<stdio.h>
#define a 2000
#define b 2000

int A[a], Mi[b];
int hantei(int, int);

int main()
{
  int n, q, i, j, k;
  // int A[a], Mi[b];

  scanf("%d", &n);
  for(i=0; i<n; i++)
    {
      scanf("%d", &A[i]);
    }

  scanf("%d", &q);
  for(j=0; j<q; j++)
    {
      scanf("%d", &Mi[j]);
    }

  for(k=0; k<q; k++)
    {
      if(hantei(n, Mi[k]))
	{
	  printf("yes\n");
	}
      else
	{
	  printf("no\n");
	}	
    }

  return 0;
}

int hantei(int v, int w)
{
  int x;
  if(w == A[v])
    {
      return 1;
    }
  if(v == -1)
    {
      return 0;
    }
  if(w <= 0)
    {
      return 0;
    }
  if(hantei(v-1,w))
    {    
      return 1;
    }
  if(hantei(v-1,w-A[v]))
    {
      return 1;
    }

  return 0;   
}