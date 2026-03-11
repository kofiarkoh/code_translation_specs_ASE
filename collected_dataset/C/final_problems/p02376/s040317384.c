#include <stdio.h>
#include <stdlib.h>

struct node{
    int to, cap; // 行き先, 容量
    struct node *next;
};
typedef struct node graph;

void add_edge(graph *g[], int u, int v, int c){
    graph *new_edge = (graph*)malloc(sizeof(graph));
    new_edge->to = v; new_edge->cap = c;
    // 新しい辺は先頭に追加していく
    new_edge->next = g[u];
    g[u] = new_edge;
}

void print_graph(graph *g[], int v){
    for(int i = 0; i <= v; i++){
        if(g[i] == NULL) continue;
        graph *tmp = g[i];
        while(tmp){
            printf("(from:%d, to:%d, cap:%d) ", i, tmp->to, tmp->cap);
            tmp = tmp->next;
        }
        printf("\n");
    }
}

void init_graph(graph *g[], int n){
    for(int i = 0; i <= n; i++){
        g[i] = NULL;
    }
}

int can_push(int height[], int from, int to, int res){
    // h(u) == h(v) + 1 && 残余容量 > 0
    return height[from] == height[to] + 1 && res > 0;
}

void push(graph *g, int from, int v, int flow[v][v], int *excess){
    int to = g->to, res = g->cap - flow[from][to];
     // 流せる最大量を求める
    int f = excess[from] < res ? excess[from] : res;
    excess[from] -= f;
    excess[to] += f;
    flow[from][to] += f;
    flow[to][from] -= f;
}

void rift(graph *g[], int from, int *height, int v, int flow[v][v]){
    graph *tmp = g[from];
    int min_h = v * 2 - 1;
    while(tmp){
        int res = tmp->cap - flow[from][tmp->to];
        if(res > 0){
            min_h = min_h < height[tmp->to] ? min_h : height[tmp->to];
        }
        tmp = tmp->next;
    }
    height[from] = min_h + 1;
}

int check(graph *g[], int s, int t, int excess[]){
    for(int i = s; i <= t; i++){
        graph *tmp = g[i];
        while(tmp){
            if(tmp->to != s && tmp->to != t && excess[tmp->to] > 0) return tmp->to;
            tmp = tmp->next;
        }
    }
    return -1;
}

void init(graph *g[], int v, int s, int *height, int *excess, int flow[v][v]){
    for(int i = 0; i < v; i++){
        height[i] = 0; excess[i] = 0;
        for(int j = 0; j < v; j++) flow[i][j] = 0;
    }
    height[s] = v;
    graph *tmp = g[s];
    while(tmp){
        excess[s] += tmp->cap;
        push(tmp, s, v, flow, excess);
        tmp = tmp->next;
    }
}

void preflow_push(graph *g[], int v, int s, int t){
    int height[v], excess[v], flow[v][v];
    init(g, v, s, height, excess, flow);
    int cur = check(g, s, t, excess);
    while(cur != -1){
        graph *tmp = g[cur];
        while(tmp){
            if(can_push(height, cur, tmp->to, tmp->cap - flow[cur][tmp->to]) != 0) break;
            tmp = tmp->next;
        }
        if(tmp != NULL) push(tmp, cur, v, flow, excess);
        else rift(g, cur, height, v, flow);
        cur = check(g, s, t, excess);
    }
    printf("%d\n", excess[t]);
}

int main(){
    int v, e; // v -> 頂点数, e -> 辺数
    scanf("%d %d", &v, &e);
    graph *g[v];
    init_graph(g, v);
    int s, t, c;
    for(int i = 0; i < e; i++){
        scanf("%d %d %d", &s, &t, &c);
        //s--; t--;
        add_edge(g, s, t, c); // 辺を追加
        add_edge(g, t, s, 0); // 辺を追加
    }
    preflow_push(g, v, 0, v - 1);
    // print_graph(g, v);
    return 0;
}

