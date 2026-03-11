#include<stdio.h>
 
int main(){
  int i, j, n, m[100][100], p[100], a, l, k, q=0;
 
  scanf("%d", &n);
  for(i = 0; i < n; i++){
    if(i < n - 1) scanf("%d%d", &p[i], &a);
    else scanf("%d%d", &p[i], &p[i + 1]);
  }
 
  for(i = 1; i <= n; i++){
    for(j = 1; j <= n; j++){
      m[i][j] = 0;
    }
  }
 
  for(l = 2; l <= n; l++){
    for(i = 1; i <= n - l + 1; i++){
      j = i + l - 1;
      m[i][j] = 100000;
      for(k = i; k < j; k++){
    q = m[i][k] + m[k + 1][j] + p[i - 1]*p[k]*p[j];
    if(m[i][j] > q) m[i][j] = q;
      }
    }
  }
 
  printf("%d\n", m[1][n]);
 
  return 0;
}