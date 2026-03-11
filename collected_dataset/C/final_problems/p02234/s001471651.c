#include <stdio.h>

#define N 100
#define INF 1000000

void Matrix_Chain_Order(int *);
int Mini(int , int );

int r[N], c[N], p[N], mat[N][N], n;

int main() {
  
  int i,j; 
  
  scanf("%d", &n);
  if(n < 0 || N < n){
    return 0;
  }
  
  for(i = 0; i < n; i++){
    scanf("%d %d", &r[i], &c[i]);
  }
  
  for(i = 0; i < n; i++){
    if(i == 0) {
      p[i] = r[i];
      p[i+1] = c[i];
    }
    else p[i+1] = c[i];
  }
  
  Matrix_Chain_Order(p);
  
  printf("%d\n", mat[1][n]);
  
  return 0;
}


void Matrix_Chain_Order(int *p){
  int i,j,k;
  int l,q=0;
  
  for(i=1;i <= n;i++){
    mat[i][i] = 0;
  }
  
  for(l = 2; l <= n; l++){
    for(i = 1; i <= n-l+1; i++){

      j = i+l-1;
      mat[i][j] = INF;

      for(k = i+1; k <= j; k++){
	q = mat[i][k-1]+mat[k][j]+(p[i-1]*p[k-1]*p[j]);
	if(q < mat[i][j]){
	  mat[i][j] = q;
	}
      }

    }
  }
}