#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>
typedef long long ll;
#define rep(i,n) for(ll i=0; i<(ll)(n); i++)
#define REP(i,l,r) for(ll i=l; i<(ll)(r); i++)
#define max(l,r) ((l)>(r)?l:r)
#define min(l,r) ((l)<(r)?l:r)
#define swap(l,r) {ll tp=l;l=r;r=tp;}
const ll MOD = 1000000007;
const int INF = (1 << 30);
int upcmpii(const void *a, const void *b){
    return (*(int*)a == *(int*)b ? 0 : (*(int*)a > *(int*)b ? 1 : -1));
}
void sort_ui(int *a, int n){ qsort(a, n, sizeof(int), upcmpii);}

#define E_MAX 202020
#define V_MAX 202020
typedef struct edlst{
    int fr, to;
}E;
typedef struct graph{
    E e[E_MAX*2];
    int st[V_MAX], siz[V_MAX];
}G;
G g;
int upe(const void *a, const void *b){
    if(((E*)a)->fr != ((E*)b)->fr) 
        return (((E*)a)->fr > ((E*)b)->fr ? 1 : -1);
    else
        return (((E*)a)->to == ((E*)b)->to ? 0 : (((E*)a)->to > ((E*)b)->to ? 1 : -1));
}
void g_init(int vn, int en){
    int a, b;
    rep(i, vn) g.siz[i] = 0;
    rep(i, en) {
        scanf("%d%d", &a, &b);
        a--; b--;
        g.e[2*i].fr = a; g.e[2*i].to = b;
        g.e[2*i+1].fr = b; g.e[2*i+1].to = a;
        g.siz[a]++; g.siz[b]++;
    }
    qsort(g.e, 2*en, sizeof(E), upe);
    g.st[0] = 0;
    rep(i, vn-1){
        g.st[i+1] = g.st[i] + g.siz[i];
    }
}

int n, q;
ll scr[202020] = {0}, ans[202020] = {0};
bool ch[202020] = {false};
void dfs(int vi, ll sum) {
    ch[vi] = true;
    ans[vi] = sum + scr[vi];
    rep(i, g.siz[vi]) {
        if(!ch[g.e[g.st[vi] + i].to]) {
            dfs(g.e[g.st[vi] + i].to, ans[vi]);
        }
    }
}

void solve() {
    int p, x;
    rep(i, q) {
        scanf("%d%d", &p, &x);
        scr[p-1] += x;
    }
    dfs(0, 0);
}

int main(){
    scanf("%d%d", &n, &q);
    g_init(n, n-1);
    solve();
    rep(i, n) {
        printf("%lld ", ans[i]);
    }
    puts("");
}