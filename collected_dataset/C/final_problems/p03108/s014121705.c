#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#define BUF_SIZE 20

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

struct pairs {
    int v1;
    int v2;
};

#define BRIDGES_MAX 100000
#define LANDS_MAX 100000

struct vertex {
    int parent;
    int depth;
    int num;
};

void init_ufind(struct vertex ufind[], int size) {
    int i;
    for(i = 0; i < size; i++) {
        ufind[i].parent = i;
        ufind[i].depth = 1;
        ufind[i].num = 1;
    }
    return;
}

int get_root(struct vertex ufind[], int v) {
    int base = v;
    struct vertex *vertex = &ufind[v];
    while(base != vertex->parent) {
        base = vertex->parent;
        vertex = &ufind[base];
    }
    return base;
}

// res: diff
uint64_t connect(struct vertex ufind[], int v1, int v2) {
    int rv1 = get_root(ufind, v1);
    int rv2 = get_root(ufind, v2);
    if(rv1 == rv2) return 0;
    // ensure that v1 and v2 are separated with each other.
    struct vertex *root1 = &ufind[rv1];
    struct vertex *root2 = &ufind[rv2];
    int cnt1 = root1->num;
    int cnt2 = root2->num;
#ifdef DEBUG
    printf("connect: root%d: %d, root%d: %d\n", rv1, cnt1, rv2, cnt2);
#endif
    if(root1->depth >= root2->depth) { // root1 <- root2
        if(root1->depth == root2->depth) {
            root1->depth++;
        }
        // update root2
        root2->depth = root1->depth;
        root2->parent = rv1;
        root1->num += root2->num;
    } else { // root2 <- root1
        root1->depth = root2->depth;
        root1->parent = rv2;
        root2->num += root1->num;
    }
    // nC2 - (n1_C_2 + n2_C_2)
    return (uint64_t)cnt1 * cnt2;
}

int main(void) {
    int lands, bridges;
    struct pairs ps[BRIDGES_MAX];
    get_int2(&lands, &bridges);

    int i;
    for(i = 0; i < bridges; i++) {
        int v1, v2;
        get_int2(&v1, &v2);
        // 1-indexed -> 0-indexed
        v1--; v2--;
        ps[i].v1 = v1; ps[i].v2 = v2;
    }

    // conn[<# of removed bridges>] = <# of connected>
    uint64_t conn[BRIDGES_MAX+1];
    struct vertex ufind[LANDS_MAX];
    init_ufind(ufind, lands);
    // None of the bridges are removed
    conn[0] = ((uint64_t)lands)*(lands-1)/2;
    conn[bridges] = 0; // all bridges are removed..
    int br;
    for(br = bridges-1; br >= 1; br--) {
        uint64_t diff = connect(ufind, ps[br].v1, ps[br].v2);
#ifdef DEBUG
        printf("[%d<->%d]diff: %llu\n", ps[br].v1, ps[br].v2,diff);
#endif
        conn[br] = conn[br+1] + diff;
    }
    
    uint64_t sum = conn[0];
    for(i = 1; i <= bridges; i++) {
        printf("%llu\n", sum - conn[i]);
    }
    return 0;
}