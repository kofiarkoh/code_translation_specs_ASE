#include <stdio.h>
#include <stdlib.h>
#define MAX_NODES 30001
#define MAX_V 10001

typedef struct edge {
    int from, to, cost;
    struct edge *next;
}edge;

typedef struct vertex{
    int value, edges;
    struct edge *head;
}vertex;

int scc(int v);
void init(int v);
void dfs(int v);
int rdfs(int i, int k);
void r_edge(int from, int to, int cost);
void add_edge(int from, int to, int cost);

vertex graph[MAX_NODES];
vertex rgraph[MAX_NODES];
int used[MAX_V];
int dfs_list[MAX_V];
int scc_ret[MAX_V];
int dfs_count;


int main(void)
{
    int v, e, i, q, tmpa, tmpb;
    scanf("%d",&v);
    scanf("%d",&e);
    init(v);
    for (i = 0; i < e; i++) {
        scanf("%d%d", &tmpa, &tmpb);
        add_edge(tmpa, tmpb, 1);
        r_edge(tmpb, tmpa, 1);
    }
/*    puts("張った");
    for (i = 0; i < v; i++) {
        edge *tmp = rgraph[i].head;
        while (tmp != NULL) {
            printf("%d %d\n", tmp->from, tmp->to);
            tmp = tmp->next;
        }
    }*/
    scc(v);
    //puts("End SCC");
    scanf("%d", &q);
    for (i = 0; i < q; i++) {
        scanf("%d%d", &tmpa, &tmpb);
        if (scc_ret[tmpa] == scc_ret[tmpb]) {
            puts("1");
        } else {
            puts("0");
        }
    }
    return 0;
}


int scc(int v)
{
    int i, k = 0;
    for (i = 0; i < v; i++) {
        used[i] = 0;
        dfs_list[i] = 0;
        scc_ret[i] = 0;
    }
    dfs_count = 0;
    for (i = 0; i < v; i++) {
        if (!used[i]) {
            dfs(i);
            //printf("dfs_end : %d\n", i);
        }
    }
    for (i = 0; i < v; i++) {
        //printf("%d\n", dfs_list[i]);
        used[i] = 0;
    }
    for (i = dfs_count - 1; i >= 0; i--) {
        if (!used[dfs_list[i]]) {
            //printf("do rdfs : %d\n", dfs_list[i]);
            rdfs(dfs_list[i], k++);
            //printf("rdfs_end : %d\n", i);
        }
    }
    return k;
}

int rdfs(int i, int k)
{
    edge *pos = rgraph[i].head;
    used[i] = 1;
    scc_ret[i] = k;
    while (pos) {
        if (!used[pos->to]) {
            //printf("do rdfs : %d\n", pos->to);
            rdfs(pos->to, k);
        }
        pos = pos->next;
    }
    return k;
}

void dfs(int i)
{
    edge *pos = graph[i].head;
    used[i] = 1;
    while (pos) {
        if (!used[pos->to]) {
            //printf("do dfs : %d\n", pos->to);
            dfs(pos->to);
        }
        pos = pos->next;
    }
    dfs_list[dfs_count++] = i;
}

void add_edge(int from, int to, int cost)
{
    //fromからtoへ辺を張る
    edge *new, *now = graph[from].head;
    new = (edge *)malloc(sizeof(edge));
    new->from = from;
    new->to = to;
    new->cost = cost;
    if (now == NULL) {
        graph[from].head = new;
    } else {
        while (now->next != NULL) {
            now = now->next;
        }
        now->next = new;
    }
    graph[from].edges++;
}

void r_edge(int from, int to, int cost)
{
    //逆順のグラフを作るためにfromからtoへ辺を張る
    edge *new, *now = rgraph[from].head;
    new = (edge *)malloc(sizeof(edge));
    new->from = from;
    new->to = to;
    new->cost = cost;
    if (now == NULL) {
        rgraph[from].head = new;
    } else {
        while (now->next != NULL) {
            now = now->next;
        }
        now->next = new;
    }
    rgraph[from].edges++;
}

void init(int v)
{       //グラフの貼り直し
    int i, j;
    vertex nv;
    nv.head = NULL;
    for (i = 0; i < v; i++) {
        graph[i] = nv;
        rgraph[i] = nv;
        used[i] = 0;
    }
}