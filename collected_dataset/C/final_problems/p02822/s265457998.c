#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // uint64_t

#define BUF_SIZE 30

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

#define VERTEX_MAX 200000
#define POWER_MAX (VERTEX_MAX*2)
#define LPOOL_MAX (VERTEX_MAX*2+10)
#define DIVISOR 1000000007

struct list {
    int dst;
    struct list *next;
};

struct list hpool[LPOOL_MAX];
int lpos = 0;

struct list *create_graph(int vs) {
    struct list *graph = &hpool[lpos];
    lpos += vs;
    int i;
    for(i = 0; i < vs; i++) {
        graph[i].next = &graph[i];
    }
    return graph;
}

void add_list(struct list *head, int dst) {
    struct list *elem = &hpool[lpos++];
    elem->dst = dst;
    elem->next = head->next;
    head->next = elem;
}

struct pair {
    int x;
    int y;
};

// assume that a <= b
struct pair gcdext(int a, int b) {
    // 0*x + 1*y = 1
    if(a == 0) { return (struct pair){0, 1}; }
    struct pair p = gcdext(b%a, a);
    return (struct pair){p.y - (b/a)*p.x, p.x};
}

/* stack data structure */
static int stack[VERTEX_MAX];
static int pos;

void push(int v) {
    stack[pos++] = v;
}

int pop(void) {
    return stack[--pos];
}

int peek(void) {
    return stack[pos-1];
}

int is_empty(void) {
    return !pos;
}

#define min(a,b) ((a) > (b) ? (b) : (a))

int main(void) {
    int vs = get_int();
    int i;

    struct list *graph = create_graph(vs);
    static struct pair edges[VERTEX_MAX];
    static int map[VERTEX_MAX+1] = {0};
    int es = vs - 1;
    for(i = 0; i < es; i++) {
        int src, dst;
        get_int2(&src, &dst);
        // 1-indexed -> 0-indexed
        src--; dst--;
        struct pair e = {src, dst};
        edges[i] = e;

        map[src]++; map[dst]++;
        add_list(&graph[src], dst);
        add_list(&graph[dst], src);
    }

    // memoize 2**i and (2)**(-i) mod DIVISOR
    static uint64_t power2[POWER_MAX+1];
    static uint64_t inv_power2[POWER_MAX+1];
    struct pair p = gcdext(2, DIVISOR); // 1/2
    int inv = (p.x + DIVISOR)%DIVISOR; // 1/2 (mod DIV)
    power2[0] = 1; inv_power2[0] = 1;
    for(i = 1; i <= POWER_MAX; i++) {
        power2[i] = (power2[i-1]*2)%DIVISOR;
        inv_power2[i] = (inv_power2[i-1]*inv)%DIVISOR;
    }

    int start = 0;
    // find root
    for(i = 0; i < vs; i++) {
        if(map[i] != 1) continue;
        start = i; break;
    }

    static int checked[VERTEX_MAX];
    static int order[VERTEX_MAX];
    struct list *cached[VERTEX_MAX];
    for(i = 0; i < vs; i++) {
        cached[i] = &graph[i];
    }
    // # of node below the root:i
    static int cnt[VERTEX_MAX];
    struct list *elem;
    int oidx = 0;
    push(start);
    checked[start] = 1;
    order[start] = oidx++;
    while(!is_empty()) {
        int node = peek();
        struct list *head = &graph[node];
        int flag = 0;
        for(elem = cached[node]->next; elem != head; elem = elem->next) {
            int dst = elem->dst;
            if(checked[dst]) continue;
            cached[node] = elem;
            order[dst] = oidx++;
            push(dst); checked[dst] = 1;
            flag = 1; break;
        }
        if(!flag) {
            cnt[node] = oidx - order[node];
            pop(); // trash
        }
    }

    // calc expeced value
    // calc p1 = (2**n-1)*(2**(vs-n)-1)
    // calc p2 = 2**vs
    uint64_t ans = -1; //  empty graph
    for(i = 0; i < es; i++) {
        uint64_t res = 1;
        int src = edges[i].x;
        int dst = edges[i].y;
        int n = min(cnt[src], cnt[dst]);
        res = (res * (power2[n]-1))%DIVISOR;
        res = (res * (power2[vs-n]-1))%DIVISOR;
        ans = (ans + res)%DIVISOR;
    }
    // sum(es, p1*p2) - 1/(2**vs) - (N-2)*2
#ifdef DEBUG
    printf("S's edge: %llu / (2**%d)\n", ans, vs);
#endif
    ans = (ans * inv_power2[vs])%DIVISOR;
    int blacks = (vs-2);
    blacks = (blacks * inv_power2[1])%DIVISOR;
    ans = (ans - blacks + DIVISOR)%DIVISOR;

    // ans must be less than DIVISOR < INT_MAX
    printf("%d\n", (int)ans);

    return 0;
}