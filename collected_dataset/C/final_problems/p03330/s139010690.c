#include<stdio.h>

int main(void){
  int n , c;
  scanf("%d %d",&n ,&c);

  int d[c+1][c+1];
  for(int i = 1;i <= c;i ++){
    for(int j = 1;j <= c;j ++){
      scanf("%d",&d[i][j]);
    }
  }

  int k[n+1][n+1];
  int waru1[31] = {0} , waru2[31] = {0} , waru0[31] = {0};
  for(int i = 1;i <= n;i ++){
    for(int j = 1;j <= n;j ++){
      scanf("%d",&k[i][j]);
      if((i+j)%3 == 1){
        waru1[k[i][j]] ++;
      }
      else if((i+j)%3 == 2){
        waru2[k[i][j]] ++;
      }
      else{
        waru0[k[i][j]] ++;
      }
    }
  }

  /*while(flag == 1){
    flag = 0;
    for(int i = 1;i <= c;i ++){
      for(int j = 1;j <= c;j ++){
        for(int k = 1;k <= c;k ++){
          if(d[i][j] > d[i][k] + d[k][j]){
            d[i][j] = d[i][k] + d[k][j];
            flag = 1;
          }
        }
      }
    }
  }*/

  int min = 1000000000;
  for(int i = 1;i <= c;i ++){
    for(int j = 1;j <= c;j ++){
      for(int x = 1;x <= c;x ++){
        if(i != j && j != x && x != i){
          int tas = 0;
          for(int y = 1;y <= c;y ++){
            tas += waru1[y] * d[y][i];
            tas += waru2[y] * d[y][j];
            tas += waru0[y] * d[y][x];
          }
          
          if(min > tas && waru1[i] + waru2[j] + waru0[x] < n*n){
            min = tas;
          }
        }
      }
    }
  }

  printf("%d\n",min);

  return 0;
}