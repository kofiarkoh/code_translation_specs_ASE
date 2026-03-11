// AOJ 2352 Divisor
// 2018.4.10 bal4u

#include <stdio.h>
#include <string.h>

#define MAX 202
int   hi[MAX],  to[MAX][MAX];
char seen[MAX];
int  match[MAX];

int bpm(int u);
int bipartiteMatching(int m, int n)
{
    int u, max;

    memset(match, -1, (m+n)*sizeof(int));
    max = 0;
    for (u = 0; u < m; u++) {
        memset(seen, 0, m+n);
        if (bpm(u)) max++;
    }
    return max;
}

int bpm(int u)
{
    int i, v;

    for (i = 0; i < hi[u]; i++) {
        v = to[u][i];
        if (seen[v]) continue;
        seen[v] = 1;
        if (match[v] < 0 || bpm(match[v])) {
            match[u] = v, match[v] = u;
            return 1;
        }
    }
    return 0;
}

int in()
{
    int n = 0, c = getchar_unlocked();
    do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
    return n;
}

int  x[102]; int N;
char f[102]; int sz;
int  a[102];

int calc(int n, int *a)
{
    int i, j;

    memset(hi, 0, (sizeof(int)*n)<<1);
    for (i = 0; i < n; i++) {
        for (j = 0; j < n; j++) if (j != i) {
            if (a[i] % a[j] == 0) to[i][hi[i]++] = n+j;
        }
    }
    return n-bipartiteMatching(n, n);
}

int main()
{
    int i, j, k, ans;

    N = in();
    for (i = 0; i < N; i++) x[i] = in();

    // 課題文の条件を満たす集合Sの大きさを確定させる
    ans = calc(N, x);

    if (ans == 1) { puts("1"); return 0; }      // Sの大きさは1の場合、無条件にx[0]が答え

    memset(f, 1, N);    // f[i]=1：答えかその候補者、0：除外すべきx[i]

    sz = 0;             // 先頭x[0]から一つずつ確定していく。確定したx[i]の個数はsz
    for (i = 0; i < N; i++) {
        // x[i] を答えの集合Sに入れることができるか
        for (j = 0; j < i; j++) if (f[j]) {
            if (!(x[i] % x[j]) || !(x[j] % x[i])) break;
        }
        if (j < i) { f[i] = 0; continue; }      // 確定した集合Sの他メンバーと衝突するので不適合

        // x[i] を集合Sのメンバーにする
        k = 0; for (j = i+1; j < N; j++) if (f[j]) {
            if (x[i] % x[j] && x[j] % x[i]) {   // 残りのx[j]からx[i]と衝突しないものを選ぶ
                a[k++] = x[j];
            } else f[j] = 2;
        }
        
                                                // calc(k, a) は残りのx[j]で作れる集合の大きさ
        if (calc(k, a) + sz + 1 == ans) {       // 確定した集合Sの大きさ（=ans）と同じ
            // 結論として、x[i]を集合Sに入れるべきだ
            if (++sz == ans) break;
            for (j = i+1; j < N; j++) if (f[j] == 2) f[j] = 0;
        } else {
            // 結論として、x[i] は不適切で除外すべきだ
            f[i] = 0;
            for (j = i+1; j < N; j++) if (f[j] == 2) f[j] = 1;
        }
    }
    memset(f+i+1, 0, N-(i+1));                  // 残りのx[i]をすべて除外

    // 確定したx[i]を出力して終了
    k = 0; for (i = 0; i < N; i++) if (f[i]) {
        if (k) putchar(' '); else k = 1;
        printf("%d", i+1);
    }
    putchar('\n');
    return 0;
}

