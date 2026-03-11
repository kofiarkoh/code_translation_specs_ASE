#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // int64_t

#define max(a,b) ((a) > (b) ? (a) : (b))
#define min(a,b) ((a) > (b) ? (b) : (a))

int get_int(void) {
  int num;
  scanf("%d", &num);
  return num;
}

int get_int2(int *a1, int *a2) {
  scanf("%d %d", a1, a2);
  return 0;
}


#define VERTEX_MAX 100000
// undirected
#define EDGE_MAX (100000*2)
struct list {
    struct list *next;
    int dst;
};

struct list *create_graph(int vs) {
    static struct list graph[VERTEX_MAX];
    int i;
    for(i = 0; i < vs; i++) {
        graph[i].next = &graph[i];
    }
    return graph;
}

struct list lpool[EDGE_MAX];
int lidx = 0;

void add_list(struct list *head, int v) {
    struct list *elem = &lpool[lidx++];
    elem->dst = v;
    elem->next = head->next;
    head->next = elem;
}

void get_dist(struct list *graph, int node, int parent, int *out) {
    if(parent == -1) {
        out[node] = 0;
    } else {
        out[node] = out[parent]+1;
    }
    struct list *elem;
    struct list *head = &graph[node];
    for(elem = head->next; elem != head; elem = elem->next) {
        if(elem->dst == parent) continue;
        get_dist(graph, elem->dst, node, out);
    }
    return;
}

int get_count(struct list *graph, int node, int parent) {
    int ans = 1;
    struct list *elem;
    struct list *head = &graph[node];
    for(elem = head->next; elem != head; elem = elem->next) {
        if(elem->dst == parent) continue;
        ans += get_count(graph, elem->dst, node);
    }
    return ans;
}


int main(void) {
    int num = get_int();
    struct list *graph = create_graph(num);
    int i;
    for(i = 0; i < num-1; i++) {
        int v1, v2;
        get_int2(&v1, &v2);
        v1--; v2--;
        add_list(&graph[v1], v2);
        add_list(&graph[v2], v1);
    }

    static int aout[VERTEX_MAX];
    static int bout[VERTEX_MAX];
    get_dist(graph, 0, -1, aout);
    get_dist(graph, num-1, -1, bout);

    int root = 0;
    // 0: 1-st, 1: 2nd
    int next_turn = 0;
    int sum = VERTEX_MAX*2;
    for(i = 0; i < num; i++) {
#ifdef DEBUG
        printf("%d -> %d : %d\n", i, aout[i], bout[i]);
#endif
        int diff = bout[i] - aout[i];
        if(sum >= aout[i]+bout[i]) {
            if(diff >= 0 && diff <= 1) {
                next_turn = (1-diff);
                root = i;
                sum = aout[i]+bout[i];
            }
            if(diff == 0) break;
        }
    }
#ifdef DEBUG
    printf("root: %d -> next: %d\n", root, next_turn);
#endif
    struct list *elem;
    struct list *head = &graph[root];
    static int blacks, whites;
    int flag = 1;
    if(aout[root] == 0) {
        // root: 0, parent: num-1
        blacks = get_count(graph, 0, num-1);
        whites = num-blacks;
        flag = (blacks <= whites);
#ifdef DEBUG
        printf("[next: %d] %d vs %d\n", next_turn, blacks, whites);
#endif
        goto finish;
    }
    for(elem = head->next; elem != head; elem=elem->next) {
        if(next_turn==0) {
            if(aout[root] > aout[elem->dst]) {
                blacks = get_count(graph, elem->dst, root)+1;
                whites = num-blacks;
            }
        // diff = 0
        } else {
            if(bout[root] > bout[elem->dst]) {
                whites = get_count(graph, elem->dst, root);
                blacks = num-whites;
            }
        }
        flag = (blacks <= whites);
    }
#ifdef DEBUG
    printf("[next: %d] %d vs %d\n", next_turn, blacks, whites);
#endif
finish:
    printf("%s\n", flag ? "Snuke" : "Fennec");
    return 0;
}