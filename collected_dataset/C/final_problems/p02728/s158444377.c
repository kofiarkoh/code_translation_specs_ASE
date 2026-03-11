#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // uint64_t

#define BUF_SIZE 30

// size: specify sizeof(str)
int get_str(char *str, int size) {
    if(!fgets(str, size, stdin)) return -1;
    return 0;
}

int get_int(void) {
  int num;
#ifdef BUF_SIZE
  char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, stdin)) return 0;
  sscanf(line, "%d", &num);
#else
#error
#endif
  return num;
}

int get_int2(int *a1, int *a2) {
#ifdef BUF_SIZE
  char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, stdin)) return -1;
  sscanf(line, "%d %d", a1, a2);
#else
#error
#endif
  return 0;
}

#define NUM_MAX 200000

struct list {
    int dst;
    struct list *next;
};

#define LPOOL_MAX (NUM_MAX*3+10)
static struct list lpool[LPOOL_MAX];
static int lpos = 0;
#define DIVISOR 1000000007

struct list *create_graph(int num) {
    struct list *graph = &lpool[lpos];
    lpos += num;
    int i;
    for(i = 0; i < num; i++) {
        graph[i].next = &graph[i];
    }
    return graph;
}

void add_list(struct list *head, int v) {
    struct list *elem = &lpool[lpos++];
    elem->dst = v;
    elem->next = head->next;
    head->next = elem;
}

static uint64_t factorial[NUM_MAX+1];
static uint64_t inv_factorial[NUM_MAX+1];

struct pair {
    int x;
    int y;
};

struct pair gcdext(int a, int b) {
    if(a == 0) { return (struct pair){0, 1}; }
    struct pair p = gcdext(b%a, a);
    return (struct pair){p.y - (b/a)*p.x, p.x};
}

void prep_factorial(int num) {
    factorial[0] = factorial[1] = 1;
    inv_factorial[0] = inv_factorial[1] = 1;
    int i;
    for(i = 2; i <= num; i++) {
        factorial[i] = (factorial[i-1]*i)%DIVISOR;
        struct pair p = gcdext(i, DIVISOR);
        int inv = (p.x + DIVISOR)%DIVISOR;
        inv_factorial[i] = (inv_factorial[i-1]*inv)%DIVISOR;
    }
}

uint64_t calc_combi(int num, int p) {
    uint64_t res = 1;
    res = (res * factorial[num])%DIVISOR;
    res = (res * inv_factorial[num-p])%DIVISOR;
    res = (res * inv_factorial[p])%DIVISOR;
    return res;
}


struct info {
    uint64_t res;
    int cnt;
};

struct info ans[NUM_MAX];

void dfs0(struct list *graph, int this, int parent) {
    struct list *head = &graph[this];
    struct list *elem;
    int cnt = 1; // myself
    for(elem = head->next; elem != head; elem = elem->next) {
        int dst = elem->dst;
        if(dst == parent) continue;
        dfs0(graph, dst, this);
        cnt += ans[dst].cnt;
    }
    uint64_t res = 1;
    int rem = cnt - 1;
    for(elem = head->next; elem != head; elem = elem->next) {
        int dst = elem->dst;
        if(dst == parent) continue;
        res = (res * calc_combi(rem, ans[dst].cnt))%DIVISOR;
        res = (res * ans[dst].res)%DIVISOR;
        rem = rem - ans[dst].cnt;
    }
    ans[this] = (struct info){res, cnt};
    return;
}

// uint64_t res = (ans[parent].res / (combi(ans[parent].cnt-1, ans[this].cnt)*ans[this].res))%DIVISOR;
struct info get_exclude_info(uint64_t whole, struct info *exclude, int vs) {
    uint64_t denom = (calc_combi(vs-1, exclude->cnt)*exclude->res)%DIVISOR;
    struct pair p = gcdext(denom, DIVISOR);
    int inv = (p.x + DIVISOR)%DIVISOR;
    uint64_t res = (whole * inv)%DIVISOR;
    return (struct info){res, vs - exclude->cnt};
}

void dfs_reroot(struct list *graph, int vs, int this, int parent) {
    if(parent != -1) {
        struct info info = get_exclude_info(ans[parent].res, &ans[this], vs);
        // update ans[this]
        ans[this].res = (ans[this].res * calc_combi(vs-1, info.cnt))%DIVISOR;
        ans[this].res = (ans[this].res * info.res)%DIVISOR;
        ans[this].cnt = vs;
    }
    struct list *head = &graph[this];
    struct list *elem;
    for(elem = head->next; elem != head; elem = elem->next) {
        int dst = elem->dst;
        if(dst == parent) continue;
        dfs_reroot(graph, vs, dst, this);
    }
    return;
}

int main(void) {
    int vs = get_int();
    int es = vs-1;
    int i;
    struct list *graph = create_graph(vs);
    prep_factorial(es);

    for(i = 0; i < es; i++) {
        int v1, v2;
        get_int2(&v1, &v2);
        // 1-indexed -> 0-indexed
        v1--; v2--;
        add_list(&graph[v1], v2);
        add_list(&graph[v2], v1);
    }
    dfs0(graph, 0, -1);
#ifdef DEBUG
    for(i = 0; i < vs; i++) {
        printf("%d: %llu(cnt: %d)\n", i, ans[i].res, ans[i].cnt);
    }
#endif
    dfs_reroot(graph, vs, 0, -1);
    for(i = 0; i < vs; i++) {
        printf("%d\n", (int)ans[i].res);
    }
    return 0;
}