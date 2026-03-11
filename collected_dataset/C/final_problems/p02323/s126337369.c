#include <stdio.h>
#include <stdlib.h>


typedef struct _route route_t;

struct _route {
    int vertex_id;
    int unvisited;
    unsigned int cost;
};

route_t *_r;
int visited_mask;
int route_index;

void route_init(int vertex_count, int route_count) {
    int i;
    visited_mask = 0;
    for (i = 0; i < vertex_count; i++) {
        visited_mask |= 1 << i;
    }

    route_index = 0;
    _r = (route_t*)malloc(sizeof(route_t) * route_count);
}

void route_delete() {
    free(_r);
}

void route_visit(route_t *r, int id) {
    r->unvisited &= ~(1 << id);
}

int route_unvisited(route_t *r, int id) {
    return r->unvisited & (1 << id);
}

route_t* route_next() {
    route_t *r;
    r = &_r[route_index++];
    return r;
}

typedef struct _binheap binheap_t;

struct _binheap {
    route_t **e;
    int count;
};


binheap_t *bh_new(int size) {
    binheap_t *bh;
    bh = (binheap_t*)malloc(sizeof(binheap_t));
    bh->e = (route_t**)malloc(sizeof(route_t*) * size);
    bh->count = 0;
    return bh;
}

void bh_delete(binheap_t *bh) {
    free(bh->e);
    free(bh);
}

void bh_percup(binheap_t *bh, int i) {
    route_t *vp, *vc, *tmp;
    while (i / 2 > 0) {
        vc = bh->e[i];
        vp = bh->e[i/2];
        if (vc->cost < vp->cost) { // parent is larger
            bh->e[i/2] = vc;
            bh->e[i] = vp;
        }
        i = i / 2;
    }
}

void bh_push(binheap_t *bh, route_t *v) {
    bh->count++;
    bh->e[bh->count] = v;
    bh_percup(bh, bh->count);
}

int bh_minchild(binheap_t *bh, int i) {
    route_t *vl, *vr;
    if (i * 2 + 1 > bh->count) {
        return i * 2;
    }
    else {
        vl = bh->e[i * 2];
        vr = bh->e[i * 2 + 1];
        if (vl->cost < vr->cost)
            return i * 2;
        else
            return i * 2 + 1;
    }
}

void bh_percdown(binheap_t *bh, int i) {
    int ci;
    route_t *vp, *vc, *tmp;
    while (i*2 <= bh->count) {
        ci = bh_minchild(bh, i);
        vp = bh->e[i];
        vc = bh->e[ci];
        if (vp->cost > vc->cost) {
            bh->e[i] = vc;
            bh->e[ci] = vp;
        }
        i = ci;
    }
}

route_t *bh_pop(binheap_t *bh) {
    if (bh->count <= 0) return NULL;
    route_t *v = bh->e[1];
    bh->e[1] = bh->e[bh->count];
    bh->count--;
    bh_percdown(bh, 1);
    return v;
}

void print_bh(binheap_t *bh) {
    int i;
    route_t *v;
    printf("bh[%d]:", bh->count);
    for (i=1; i <= bh->count; i++) {
        v = bh->e[i];
        printf(" %d", v->cost);
    }
    printf("\n");
}

typedef struct _edge edge_t;

struct _edge {
    int dest;
    int distance;
    edge_t *next;
};

typedef struct _vertex vertex_t;
struct _vertex {
    int id;
    edge_t *edge;
};

#define ROUTE_COUNT 10000

int main() {
    vertex_t *vertex, *v;
    edge_t *edge, *e;
    binheap_t *bh;
    route_t *route, *r, *nr;
    int V, E, i, j, s, t, d, cost, next_v, dist;

    scanf("%d %d", &V, &E);
    vertex = (vertex_t*)malloc(sizeof(vertex_t) * V);
    edge = (edge_t*)malloc(sizeof(edge_t) * E);
    for (i = 0; i < V; i++) {
        v = &vertex[i];
        v->id = i;
        v->edge = NULL;
    }
    for (i = 0; i < E; i++) {
        scanf("%d %d %d", &s, &t, &d);
        e = &edge[i];
        e->dest = t;
        e->distance = d;
        e->next = vertex[s].edge;
        vertex[s].edge = e;
    }

    route_init(V, ROUTE_COUNT);

    bh = bh_new(ROUTE_COUNT);
    r = route_next();
    r->unvisited = visited_mask;
    route_visit(r, 0);
    r->vertex_id = 0;
    r->cost = 0;
    bh_push(bh, r);

    cost = -1;
    while (bh->count) {
        r = bh_pop(bh);
        v = &vertex[r->vertex_id];
        e = v->edge;
        while (e != NULL) {
            next_v = e->dest;
            dist = e->distance;
            if (next_v == 0) {  // last ?
                if (r->unvisited == 0) {
                    cost = r->cost + dist;
                    goto route_found;
                }
            }
            else {
                if (route_unvisited(r, next_v)) {
                    nr = route_next();
                    nr->vertex_id = next_v;
                    nr->unvisited = r->unvisited;
                    route_visit(nr, next_v);
                    nr->cost = r->cost + dist;
                    bh_push(bh, nr);
                }
            }
            e = e->next;
        }
    }
    route_found:;

    printf("%d\n", cost);

    bh_delete(bh);
    route_delete();

    return 0;
}