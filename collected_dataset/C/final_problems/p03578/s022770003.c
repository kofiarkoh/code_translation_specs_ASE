#include<stdio.h>
int main(){
  int n1, n2, d[2][200000] = {0}, a, k = 0, j;
  scanf("%d", &n1);
  for(int i = 0; i < n1; ++i){
    scanf("%d", &d[1][i]);
    d[0][i] = 1;
  }
   for (int i = 0; i < n1 - 1; i++){
    for (j = n1 - 1; j >= i + 1; j--){   //　右から左に操作
      if (d[1][j] < d[1][j-1]) { 
        int tmp = d[1][j];
        d[1][j] = d[1][j-1];
        d[1][j-1] = tmp;
      }
    }
  }
  scanf("%d", &n2);
  for(int i = 0; i < n2; ++i){
    int l = 0;
    scanf("%d", &a);
    if(k) continue;
    if(a > d[1][n1 - 1]){
      k++;
      continue;
    }
    for(j = 0; j < n1; ++j){
      while(1){
        if(j + 500 > n1) break;
        if(d[1][j + 500] < a){
          j += 501;
          continue;
        }
        break;
      }
      if(d[1][j] > a){
        k++;
        break;
      }
      if(d[0][j] == 0) continue;
      if(d[1][j] == a){
        d[0][j] = 0;
        l = 1;
        break;
      }
    }
    if(j == n1 - 1 && l == 0) k++;
  }
  if(k) puts("NO");
  else puts("YES");
  return 0;
}