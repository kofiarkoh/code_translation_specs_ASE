#include<stdio.h>
#include<stdlib.h>

int max(int a, int b){
  return (a<b?b:a);
}

int main(int argc, char *argv[])
{
  int testcase = 0;
  scanf("%d", &testcase);
  
  while(testcase--){
    
    int i, j;
    int m, n, o;
    scanf("%d%d%d", &m, &n, &o);
    int walks[1+m][1+n];
    int prohibited[1+m][1+n][2];
    
    for(i = 0; i < 1+m; ++i){
      for(j = 0; j < 1+n; ++j){
        prohibited[i][j][0] = 0;
        prohibited[i][j][1] = 0;
      }
    }
    
    for(i = 0; i < o; ++i){
      int ti, tj, tti, ttj;
      scanf("%d%d%d%d", &ti, &tj, &tti, &ttj);
      if( tj == ttj ){
        prohibited[max(ti,tti)][tj][0] = 1;
      }else if( ti == tti ){
        prohibited[ti][max(tj,ttj)][1] = 1;
      }
    }
    
    walks[0][0] = 1;
    for(i = 0; i < 1+m; ++i){
      for(j = 0; j < 1+n; ++j){
        if( i == 0 && j == 0 ) continue;
        walks[i][j] = 0;

        if( i > 0 && !prohibited[i][j][0] ){
          walks[i][j] += walks[i-1][j];
        }
        if( j > 0 && !prohibited[i][j][1] ){
          walks[i][j] += walks[i][j-1];
        }
      }
    }
    
    if( walks[m][n] == 0 ){
      puts("Miserable Hokusai!");
    }else{
      printf("%d\n", walks[m][n]);
    }

  }
  return (EXIT_SUCCESS);
}