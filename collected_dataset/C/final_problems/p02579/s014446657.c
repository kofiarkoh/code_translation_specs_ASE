#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>

typedef long long LL;

#define FOR(i,a,b) for(int i=(a); i<(b); i++)
#define REP(i,n) FOR(i,0,n)
#define max(l,r) ((l)>(r)?l:r)
#define min(l,r) ((l)<(r)?l:r)

const int MAX = 600000;
const int LIM = 100000000;
const LL MOD = 1000000007;
const int INF = 2147483647/2;
const LL LINF = 9223372036854775807/3;

int ucmpii(const void *a, const void *b){return(*(int*)a==*(int*)b?0:(*(int*)a>*(int*)b?1:-1));}
int dcmpii(const void *a, const void *b){return(*(int*)a==*(int*)b?0:(*(int*)a<*(int*)b?1:-1));}

int ucmpll(const void *a, const void *b){return(*(LL*)a==*(LL*)b?0:(*(LL*)a>*(LL*)b?1:-1));}
int dcmpll(const void *a, const void *b){return(*(LL*)a==*(LL*)b?0:(*(LL*)a<*(LL*)b?1:-1));}

void SORT(int*a, int n){qsort(a,n,sizeof(int),ucmpii);}
void SORTR(int*a, int n){qsort(a,n,sizeof(int),dcmpii);}
void LSORT(LL*a, int n){qsort(a,n,sizeof(LL),ucmpll);}
void LSORTR(LL*a, int n){qsort(a,n,sizeof(LL),dcmpll);}

const int dx[] = {1,0,-1,0,1,1,-1,-1};
const int dy[] = {0,1,0,-1,1,-1,1,-1};


int H,W;
LL ans=0;
int sx,sy,gx,gy;
char S[1010][1010];
int cnt[1010][1010] = {0};
int qxbeg=0,qxend=0, quex[100000000];
int qybeg=0,qyend=0, quey[100000000];

void dfs(int a, int b){
  cnt[a][b] = ans+1;
  REP(i,4){
    int x = a+dx[i];
    int y = b+dy[i];
    if(x<0||H<=x||y<0||W<=y) continue;
    if(S[x][y] == '#') continue;
    if(cnt[x][y]) continue;
    dfs(x,y);
  }
  REP(i,5)REP(j,5){
    int x = a-2+i;
    int y = b-2+j;
    if(x<0||H<=x||y<0||W<=y) continue;
    if(S[x][y] == '#') continue;
    if(cnt[x][y]) continue;
    quex[qxend++] = x;
    quey[qyend++] = y;
  }
}

void solve(){
  dfs(sx,sy);
  while(!cnt[gx][gy]){
    ans++;
    int N = qxend-qxbeg;
    int tmp=0;
    REP(i,N){
      int x = quex[qxbeg++], y = quey[qybeg++];
      if(cnt[x][y]==0){
        dfs(x,y);
        tmp++;
      }
    }
    if(!tmp && !cnt[gx][gy]){
      ans = -1;
      break;
    }
  }
}

int main(){
  
  scanf("%d%d%d%d%d%d",&H,&W,&sx,&sy,&gx,&gy);
  sx--;sy--;gx--;gy--;
  REP(i,H) scanf("%s",S[i]);

  solve();

  printf("%lld",ans);
}