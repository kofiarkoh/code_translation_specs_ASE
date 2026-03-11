#include <stdio.h>
#include <stdlib.h>

long *tree[100001];

int main(void) {

  long n,u,v;
  scanf("%ld %ld %ld", &n, &u, &v);
  long tree_s[n+1];
  for (long i = 0; i <= n; i++) {
    tree_s[i] = 0;
  }
  long a,b;
  for (long i = 0; i < n-1; i++) {
    scanf("%ld %ld", &a, &b);
    tree[a] = realloc(tree[a], sizeof(long)*(tree_s[a]+1));
    tree[a][tree_s[a]] = b;
    tree_s[a]++;
    tree[b] = realloc(tree[b], sizeof(long)*(tree_s[b]+1));
    tree[b][tree_s[b]] = a;
    tree_s[b]++;
  }
  int visited[n+1];
  long depth_u[n+1],depth_v[n+1];
  for (long i = 0; i <= n; i++) {
    visited[i] = 0;
    depth_u[i] = 0;
    depth_v[i] = 0;
  }
  visited[u] = 1;
  long queue[n];
  queue[0] = u;
  long start = 0, end = 0, tail = 0;
  long cur,next;
  while (tail < n-1) {
    for (long i = start; i <= end; i++) {
      cur = queue[i];
      for (long j = 0; j < tree_s[cur]; j++) {
        next = tree[cur][j];
        if (visited[next] == 1) {
          continue;
        }
        visited[next] = 1;
        depth_u[next] = depth_u[cur]+1;
        tail++;
        queue[tail] = next;
      }
    }
    start = end+1;
    end = tail;
  }
  for (long i = 0; i <= n; i++) {
    visited[i] = 0;
  }
  visited[v] = 1;
  queue[0] = v;
  start = 0; end = 0; tail = 0;
  while (tail < n-1) {
    for (long i = start; i <= end; i++) {
      cur = queue[i];
      for (long j = 0; j < tree_s[cur]; j++) {
        next = tree[cur][j];
        if (visited[next] == 1) {
          continue;
        }
        visited[next] = 1;
        depth_v[next] = depth_v[cur]+1;
        tail++;
        queue[tail] = next;
      }
    }
    start = end+1;
    end = tail;
  }
  long move;
  long max = 0;
  for (long i = 1; i <= n; i++) {
    if (depth_u[i] >= depth_v[i]) {
      continue;
    }
    move = depth_v[i]-1;
    if (move > max) {
      max = move;
    }
  }
  printf("%ld\n", max);

  return 0;
}