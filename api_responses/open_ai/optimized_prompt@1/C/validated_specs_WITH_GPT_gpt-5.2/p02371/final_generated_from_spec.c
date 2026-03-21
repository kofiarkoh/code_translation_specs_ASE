#include <stdio.h>

#define MAXN 100000
#define MAXE (2 * (MAXN - 1))

static int head[MAXN];
static int to[MAXE];
static int wgt[MAXE];
static int nxt[MAXE];
static int edge_cnt;

static int visited[MAXN];
static int distv[MAXN];

static void add_edge(int u, int v, int w) {
    to[edge_cnt] = v;
    wgt[edge_cnt] = w;
    nxt[edge_cnt] = head[u];
    head[u] = edge_cnt;
    edge_cnt++;
}

static void dfs(int start, int n) {
    /* Iterative DFS to avoid recursion depth issues; semantics match:
       visit each vertex at most once, and distv[v] is path sum from start. */
    static int stack[MAXN];
    int top = 0;

    for (int i = 0; i < n; i++) {
        visited[i] = 0;
        distv[i] = 0;
    }

    visited[start] = 1;
    distv[start] = 0;
    stack[top++] = start;

    while (top > 0) {
        int u = stack[--top];
        for (int e = head[u]; e != -1; e = nxt[e]) {
            int v = to[e];
            if (!visited[v]) {
                visited[v] = 1;
                distv[v] = distv[u] + wgt[e];
                stack[top++] = v;
            }
        }
    }
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    for (int i = 0; i < n; i++) head[i] = -1;
    edge_cnt = 0;

    for (int i = 0; i < n - 1; i++) {
        int a, b, c;
        scanf("%d %d %d", &a, &b, &c);
        add_edge(a, b, c);
        add_edge(b, a, c);
    }

    /* First traversal from 0 */
    dfs(0, n);

    /* Select farthest endpoint; tie broken by greatest vertex index */
    int endpoint = 0;
    int maxd = distv[0];
    for (int i = 0; i < n; i++) {
        if (distv[i] > maxd || (distv[i] == maxd && i > endpoint)) {
            maxd = distv[i];
            endpoint = i;
        }
    }

    /* Second traversal from selected endpoint */
    dfs(endpoint, n);

    int diameter = distv[0];
    for (int i = 0; i < n; i++) {
        if (distv[i] > diameter) diameter = distv[i];
    }

    printf("%d\n", diameter);
    return 0;
}
