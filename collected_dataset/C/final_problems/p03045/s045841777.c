#include <stdio.h>

int ans = 0;
int n, m;
int parent[100005];
int f[100005];

void init(void);
int root(int x);
void unite(int x, int y);

int main()
{
    init();
    
    int n, m;
    int x, y, z;

    scanf("%d %d", &n, &m);
    for (int i = 0; i < m; i++) {
        scanf("%d %d %d", &x, &y, &z);

        unite(x, y);
    }

    for (int i = 1; i <= n; i++) {
        int r = root(i);

        if (f[r] == 0) {
            f[r] = 1;
            ans++;
        }
    }

    printf("%d\n", ans);
    
    return 0;
}

void init(void)
{
    for (int i = 0; i < 100005; i++) {
        parent[i] = -1;
    }
}

int root(int x)
{
    if (parent[x] < 0) {
        return x;
    }

    return parent[x] = root(parent[x]);
}

void unite(int x, int y)
{
    x = root(x);
    y = root(y);

    if (x == y) return;

    parent[x] += parent[y];
    parent[y] = x;
}
