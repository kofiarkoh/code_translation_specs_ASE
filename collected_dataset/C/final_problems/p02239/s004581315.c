#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
 
typedef struct Status{
  int p, d;
} St;
 
int head, tail;
St queue[1001];
 
void qpush(St v)
{
  queue[tail++] = v;
}
 
St qpop()
{
  return queue[head++];
}
 
bool qempty()
{
  return head == tail; 
}
 
int n;
bool a[101][101];
int d[101];
 
void bfs()
{
  head = tail = 0;
  St st, st2;
  int i;
  st.p = 1;
  d[1] = st.d = 0;
 
  qpush(st);
  while(!qempty()){
    st = qpop();
 
    for(i = 1; i <= n; ++i)
      if(d[i] == -1 && a[st.p][i]){
    st2.p = i;
    d[i] = st2.d = st.d + 1;
       qpush(st2);
      }
  }
}
 
int main()
{
  int i, u, k, v;
 
  while(scanf("%d", &n) != EOF){
    memset(a, false, sizeof(a));
    memset(d, -1, sizeof(d));
 
    for(i = 0; i < n; ++i){
      scanf("%d %d", &u, &k);
 
      while(0 < k--){
    scanf("%d", &v);
    a[u][v] = true;
      }
    }
 
    bfs();
     
    for(i = 1; i <= n; ++i)
      printf("%d %d\n", i, d[i]);
  }
 
  return 0;
}