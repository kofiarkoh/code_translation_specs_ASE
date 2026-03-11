#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))
#define Min(a, b) ((a) > (b) ? (b) : (a))
#define REP(i, n) for(int i = 0; i < n; i++)
typedef long long ll;

typedef struct edge {int a, b; ll c;} E;
typedef struct graph {
  int vcnt, ecnt;
  E e[40010];
  int id[200];
} G;

int esort(const void *a, const void *b){
  E *A = (E *)a, *B = (E *)b;
  return A->a - B->a;
}

void MakeGraph(G *g, int n, int m){
  g->vcnt = n; g->ecnt = 2 * m;

  int x, y; ll z;
  REP(i, g->ecnt/2){
    scanf("%d %d %lld", &x, &y, &z);
    x--; y--;
    g->e[2 * i].a = x; g->e[2 * i].b = y; g->e[2 * i].c = z;
    g->e[2 * i + 1].a = y; g->e[2 * i + 1].b = x; g->e[2 * i + 1].c = z;
  }

  qsort(g->e, g->ecnt, sizeof(E), esort);

  int p = 0;
  REP(i, g->vcnt){
    while(p < g->ecnt && g->e[p].a < i) p++;
    g->id[i] = p;
  }
  g->id[n] = g->ecnt;
}

ll d[200][200] = {};

ll get_d(int i, int s, G *g){
  if(i == s) return 0;
  if(d[i][s] != 0) return d[i][s];
  d[i][s] = 1000000000;
  for(int j = g->id[i]; j < g->id[i + 1]; j++){
    d[i][s] = Min(d[i][s], g->e[j].c + get_d(g->e[j].b, s, g));
  }
  return d[i][s];
}

int array[10], visited[10], r[100], R;
void solve(int i, ll *ans, G *g){
  if(i == R){
    ll a = 0;
    REP(j, R - 1) a += get_d(r[array[j + 1]], r[array[j]], g);
    *ans = Min(*ans, a);
    return;
  }
  for(int j = 0; j < R; j++){
    if(visited[j] == 0){
      visited[j] = 1;
      array[i] = j;
      solve(i + 1, ans, g);
      visited[j] = 0;
    }
  }
  return;
}
      

int
main(int argc, char *argv[])
{
  int n, m; G g;
  scanf("%d %d %d", &n, &m, &R);
  REP(i, R){
    scanf("%d", &r[i]);
    r[i]--;
  }
  MakeGraph(&g, n, m);

  ll ans = 1000000000000;
  solve(0, &ans, &g);

  printf("%lld\n", ans);

  return 0;
}

  
