#include <stdio.h>
#include <stdlib.h>
 
#define WHITE 0
#define BRACK 1
 
int **matrix;
int *d;
int *parent;
int *color;
 
void prim(int);
 
int main()
{
  int n,i,j;
  int cost=0;
  scanf("%d",&n);
  
  color =  malloc(n * sizeof(int ));
  d =  malloc(n * sizeof(int ));
  parent = malloc(n * sizeof(int));
  
  matrix = malloc(n * sizeof(int *));
  for(i=0;i<n;i++)
    matrix[i] = malloc(n * sizeof(int));
  
  for(i=0;i<n;i++)
    for(j=0;j<n;j++)
      matrix[i][j] = -1;
   
  for(i=0;i<n;i++)
    for(j=0;j<n;j++)
      scanf("%d",&matrix[i][j]);
  
  prim(n);
  
  for(i=0;i<n;i++)
    cost += d[i];
  
  printf("%d\n",cost);
  
  free(d);
  free(color);
  free(parent);
  for(i=0;i<n;i++) free(matrix[i]);
  free(matrix);
 
  return 0;
   
}
 
 
void prim(int n)
{
  int mincost;
  int i,j,u;
  
  for(i=0;i<n;i++)
    {
      d[i] = 2001;
      parent[i] = -1;
      color[i] = WHITE;
    }
  
  d[0] = 0;
  
  while(1)
    {
      mincost = 2001;
      
      for(i=0;i<n;i++)
	{
	  if(color[i] != BRACK && d[i] < mincost)
	    {
	      mincost = d[i];
	      u = i;
	    }
	}
      
      if(mincost == 2001) break;
      
      color[u] = BRACK;
      
      for(i=0;i<n;i++)
	{
	  if(color[i] != BRACK && matrix[u][i] != -1 && matrix[u][i] < d[i])
	    {
	      d[i] = matrix[u][i];
	      parent[i] = u;
	    }
	}
    }
}