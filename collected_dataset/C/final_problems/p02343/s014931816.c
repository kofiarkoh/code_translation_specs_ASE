#include <stdio.h>
#define MAX_N 10000

int par[MAX_N], rank[MAX_N];

void init(int n);
int find(int x);
void unite(int x, int y);
int same(int x, int y);

void init(int n)
{
    int i;
    for (i = 0; i < n; i++) {
        par[i] = i;
        rank[i] = 0;
    }
}

int find(int x)
{
    if (par[x] == x) {
        return x;
    } else {
        return par[x] = find(par[x]);
    }
}

void unite(int x, int y)
{
    x = find(x);
    y = find(y);
    if (x == y) {
        return ;
    }
    if (rank[x] < rank[y]) {
        par[x] = y;
    } else {
        par[y] = x;
        if (rank[x] == rank[y]) {
            rank[x]++;
        }
    }
}

int same(int x, int y)
{
    //同じなら0, 違うならそれ以外
    return find(x) - find(y);
}

int main(void)
{
    int n, q, i, j, com, x, y;
    scanf("%d %d", &n, &q);
    init(n);
    for (i = 0; i < q; i++) {
        scanf("%d %d %d", &com, &x, &y);
        if (com) {
            if (same(x, y)) {
                puts("0");
            } else {
                puts("1");
            }
        } else {
            unite(x, y);
        }
    }
    return 0;
}