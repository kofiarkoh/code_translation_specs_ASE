#include<stdio.h>
#include<stdbool.h>
#include<string.h>

#define MAX 201
#define TRUE 1
#define FALSE 0
#define UNUSED -1

int N, K, E; // E : the number of edges
int graph[MAX][100010], size[MAX], match[MAX], used[MAX];

void add_edge(int s,int t) {
  graph[s][size[s]++] = t;
  graph[t][size[t]++] = s;
}

int dfs(int cur) {
  used[cur] = TRUE;
  int i, next, state;
  for(i=0;i<size[cur];++i) {
    next = graph[cur][i], state = match[next];
    if( state == UNUSED || ( !used[next] && dfs(next) ) ) {
      match[cur] = next;
      match[next] = cur;
      return TRUE;
    }
  }
  return FALSE;
}

int bmatch() {
  memset(match,UNUSED,sizeof match);
  int i, answer = 0;
  for(i=0;i<N;++i) {
    memset(used, FALSE, sizeof used);
    if( dfs(i) ) {
      ++answer;
    }
  }
  return answer;
}

int main() {
  int i,s,t;
  scanf("%d %d %d",&N,&K,&E);
  for(i=0;i<E;i++) {
    scanf("%d %d",&s,&t);
    add_edge(s,N+t);
  }
  int maxi = bmatch();
  //printf("answer = %d\n",maxi);
  printf("%d\n",maxi);
  return 0;
}