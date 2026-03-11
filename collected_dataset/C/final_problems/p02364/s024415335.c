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

int get_int3(int *a1, int *a2, int *a3) {
#ifdef BUF_SIZE
  char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, stdin)) return -1;
  sscanf(line, "%d %d %d", a1, a2, a3);
#else
#error
#endif
  return 0;
}

struct edge {
    int v1;
    int v2;
    int weigh;
};

#define VERTEX_MAX 10000
#define EDGES_MAX 100000

int asc(const void *a1, const void *a2) {
    struct edge *e1 = (struct edge*)a1;
    struct edge *e2 = (struct edge*)a2;
    return e1->weigh - e2->weigh;
}

struct ufind {
    int parent;
    int depth;
};

#define UPOOL_MAX (VERTEX_MAX+EDGES_MAX+10)
struct ufind upool[UPOOL_MAX];
int uidx;


struct ufind *create_union_find(int vs) {
    struct ufind *ufind = &upool[uidx];
    int i;
    uidx += vs;
    for(i = 0; i < vs; i++) {
        ufind[i].parent = i;
        ufind[i].depth = 1;
    }
    return ufind;
}

int get_root(struct ufind *ufind, int node) {
    struct ufind *v = &ufind[node];
    int base = node;
    while(base != v->parent) {
        // update
        base = v->parent;
        v = &ufind[base];
    }
    return base;
}

int is_same(struct ufind *ufind, int v1, int v2) {
    return get_root(ufind, v1) == get_root(ufind, v2);
}

// assume that v1 and v2 are separated.
void connect(struct ufind *ufind, int v1, int v2) {
    int b1 = get_root(ufind, v1);
    int b2 = get_root(ufind, v2);
    struct ufind *root1 = &ufind[b1];
    struct ufind *root2 = &ufind[b2];
    if(root1->depth >= root2->depth) { // root1 <- root2
        if(root1->depth == root2->depth) {
            root1->depth++;
        }
        // update
        root2->depth = root1->depth;
        root2->parent = b1;
    } else {
        // update
        root1->depth = root2->depth;
        root1->parent = b2;
    }
    return;
}

int main(void) {
    int vs, es;
    get_int2(&vs, &es);
    int i;
    struct edge edges[EDGES_MAX];
    for(i = 0; i < es; i++) {
        // 0-indexed
        get_int3(&edges[i].v1, &edges[i].v2, &edges[i].weigh);
    }

    qsort(edges, es, sizeof(struct edge), asc);
    int ans = 0;
    struct ufind *ufind = create_union_find(vs);
    int cnt = 0;
    for(i = 0 ; i< es; i++) {
        int v1 = edges[i].v1; int v2 = edges[i].v2;
        if(!is_same(ufind, v1, v2)) {
            connect(ufind, v1, v2);
            ans += edges[i].weigh;
            cnt++;
        }
        // Is tree configured?
        if(cnt == vs-1) break;
    }
    printf("%d\n", ans);
    return 0;
}
