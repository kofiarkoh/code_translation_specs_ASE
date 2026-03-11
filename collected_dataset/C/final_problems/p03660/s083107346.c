#include <stdio.h>
#define limit 100001

int N;
int count;
int cnt[limit];
int visited[limit];
int graph[limit][100];

void init(int N){
  for(int i = 0;i < N;i++) visited[i] = 0;
  count = 0;
}

void dfs(int v,int teki){
  visited[v] = 1;
  count++;
  for(int v2 = 0;v2 < cnt[v];v2++){
    if(visited[graph[v][v2]] == 1) continue;
    if(graph[v][v2] == teki) continue;
    dfs(graph[v][v2],teki);
  }
}

int main(void){
  int fennec,snuke;
  int a,b;
  scanf("%d",&N);

  for(int i = 0;i < N;i++){
    cnt[i] = 0;
    for(int j = 0;j < 100;j++){
      graph[i][j] = 0;
    }
  }
  for(int i = 0;i < N-1;i++){
    scanf("%d%d",&a,&b);
    graph[a][cnt[a]++] = b;
    graph[b][cnt[b]++] = a;
  }

  init(N);
  dfs(1,N);
  fennec = count;
  init(N);
  dfs(N,1);
  snuke = count;

  if(fennec >= snuke) printf("Fennec\n");
  else printf("Snuke\n");

  return 0;
}