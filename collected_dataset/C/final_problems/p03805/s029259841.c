#include<stdio.h>

  int n, m;
  int root = 0;
  int memo[9][9] = {0};
  int seen[9] = {0};

void dfs(int x, int y){

  if(seen[y] == 1){
    return;
  }

  seen[y] = 1;
  int flag = 0;
  for(int i = 1; i <=n ; ++i){
    if(seen[i] == 0) flag = 1;
  }
  if(flag == 0) root ++;
  flag = 0;

  for(int i = 1; i <= n; ++i){
    if( (memo[y][i] == 1) && (i != x) ){
      dfs(y,i);
    }
  }
  seen[y] = 0;
  return;
}

int main(){

  scanf("%d %d",&n, &m);
  int a,b;
  for(int i = 0; i < m; ++i){
    scanf("%d %d",&a,&b);
    memo[a][b] = 1;
    memo[b][a] = 1;
  }

  seen[1] = 1;
  for(int vertex = 2; vertex <= n; ++vertex){
    if(memo[1][vertex] == 1){
      dfs(1,vertex);
    }
  }

  printf("%d",root);
  return 0;

}
