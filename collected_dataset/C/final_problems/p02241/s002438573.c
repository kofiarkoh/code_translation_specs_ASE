#include<stdio.h>
#include<stdlib.h>

#define WHITE 0
#define BLACK 1
#define inf 2000

int main(){
  int n, mincost, sum=0;
  int A[101][101]={}, color[101], d[101], pi[101];
  int i, j, r, u, v;

  scanf("%d", &n);
  if(n < 0 || 100 < n)
    exit(1);

  for(i = 0; i < n; i++){
    for(j = 0; j < n; j++){
      scanf("%d", &A[i][j]);
      if(A[i][j] != -1 && (A[i][j] < 0 || 2000 < A[i][j]))
	exit(2);
    }
  }

  for(u = 0; u < n; u++){
    d[u] = inf;
    pi[u] = -1;
    color[u] = WHITE;
  }
  d[0] = 0;

  while(1){
    mincost = inf;
    for(i = 0; i < n; i++){
      if((color[i] != BLACK) && (d[i] < mincost)){
	  mincost = d[i];
	  u = i;
      }
    }
    if(mincost == inf)
      break;
    color[u] = BLACK;
    for(v = 0; v < n; v++){
      if((color[v] != BLACK) && (A[u][v] != -1) && (A[u][v] < d[v])){
	pi[v] = u;
	d[v] = A[u][v];
      }
    }
  }

  for(i = 0; i < n; i++)
    sum += d[i];
  
  printf("%d\n", sum);

  return 0;
}