#include <stdio.h>
#include <stdlib.h>


const int route_max = 100000;

typedef struct _edge_t edge_t;
struct _edge_t {
    int target;
    int distance;
    int slimit;
    edge_t *next;
};

typedef struct {
    int id;
    int adj_goal;
    edge_t *edge;
} vertex_t;


typedef struct {
    int current;
    int prev;
    int speed;
    double elapsed;
} route_t;

route_t *_route;
int _route_idx;
route_t *route_new(int size) {
    _route = (route_t*)malloc(size * sizeof(route_t));
    _route_idx = 0;
}

void route_reset() {
    _route_idx = 0;
}

void route_delete() {
    free(_route);
    _route = NULL;
}

route_t *route_next() {
    route_t *r;
    if (_route_idx >= route_max) {
        r = &_route[_route_idx-1];
        printf("ROUTE OVERFLOW :%d (%d, %.2lf)\n", r->current, r->speed, r->elapsed);
        exit(75);
    }
    r = &_route[_route_idx++];
    r->current = 0;
    r->prev = 0;
    r->speed = 1;
    r->elapsed = 0;
    return r;
}

typedef struct {
    route_t **e;
    int count;
    int capacity;
} binheap;


binheap *bh_new(int size) {
    binheap *bh;
    bh = (binheap*)malloc(sizeof(binheap));
    bh->e = (route_t**)malloc((size+1) * sizeof(route_t*));
    bh->count = 0;
    bh->capacity = size;
    return bh;
}

void bh_delete(binheap *bh) {
    free(bh->e);
    free(bh);
}

void bh_reset(binheap *bh) {
    bh->count = 0;
}

route_t *bh_top(binheap *bh) {
    return bh->e[1];
}

int bh_prior(route_t *a, route_t *b) {
    return a->elapsed < b->elapsed;
}

int bh_priorchild(binheap *bh, int i) {
    route_t *vl, *vr;
    if (i * 2 + 1 > bh->count) {
        return i * 2;
    }
    else {
        vl = bh->e[i * 2];
        vr = bh->e[i * 2 + 1];
        if (bh_prior(vl, vr))
            return i * 2;
        else
            return i * 2 + 1;
    }
}

void bh_percup(binheap *bh, int i) {
    route_t *vp, *vc, *tmp;
    while (i / 2 > 0) {
        vc = bh->e[i];
        vp = bh->e[i/2];
        if (bh_prior(vc, vp)) {
            bh->e[i/2] = vc;
            bh->e[i] = vp;
        }
        i = i / 2;
    }
}

void bh_percdown(binheap *bh, int i) {
    int ci;
    route_t *vp, *vc, *tmp;
    while (i*2 <= bh->count) {
        ci = bh_priorchild(bh, i);
        vp = bh->e[i];
        vc = bh->e[ci];
        if (bh_prior(vc, vp)) {
            bh->e[i] = vc;
            bh->e[ci] = vp;
        }
        i = ci;
    }
}

void bh_push(binheap *bh, route_t *v) {
    if (bh->count + 1 > bh->capacity) {
        printf("HEAP OVERFLOW\n");
        exit(75);
    }
    bh->count++;
    bh->e[bh->count] = v;
    bh_percup(bh, bh->count);
}

route_t *bh_pop(binheap *bh) {
    if (bh->count <= 0) return NULL;
    route_t *v = bh->e[1];
    bh->e[1] = bh->e[bh->count];
    bh->count--;
    bh_percdown(bh, 1);
    return v;
}



int main(void) {
    int n, m, s, g, x, y, d, c, i, k, next_speed;
    binheap *bh;
    route_t *route, *r, *nr, *fastest;
    edge_t *edge, *e;
    vertex_t vertex[31], *v;

    bh = bh_new(route_max);
    route_new(route_max);

    while (scanf("%d%d\n", &n, &m) != EOF && n) {
        int loopcheck[31][31][30] = {};
        scanf("%d%d\n", &s, &g);
        fastest = NULL;
        edge = (edge_t*)malloc((m*2) * sizeof(edge_t));
        for (i = 1; i <= n; i++) {
            v = &vertex[i];
            v->id = i;
            v->adj_goal = 0;
            v->edge = NULL;
        }
        for (i = 0, k = 0; i < m; i++) {
            scanf("%d%d%d%d\n", &x, &y, &d, &c);
            v = &vertex[x];
            e = &edge[k++];
            e->target = y;
            e->distance = d;
            e->slimit = c;
            e->next = v->edge;
            v->edge = e;
            if (y == g) v->adj_goal = 1;

            v = &vertex[y];
            e = &edge[k++];
            e->target = x;
            e->distance = d;
            e->slimit = c;
            e->next = v->edge;
            v->edge = e;
            if (x == g) v->adj_goal = 1;
        }

        bh_reset(bh);
        route_reset();
        r = route_next();
        r->current = s;
        r->prev = -1;
        r->speed = 1;
        bh_push(bh, r);
        while (bh->count) {
            r = bh_pop(bh);
            if (r->current == g && r->speed == 1) {
                fastest = r;
                break;
            }
            e = vertex[r->current].edge;
            while (e != NULL) {
                if (e->target == r->prev) goto next_edge;
                for (i = -1; i <= 1; i++) {
                    next_speed = r->speed + i;
                    if (next_speed == 0 || next_speed > e->slimit) continue;
                    if (loopcheck[r->current][e->target][next_speed]) continue;
                    loopcheck[r->current][e->target][next_speed] = 1;
                    nr = route_next();
                    nr->current = e->target;
                    nr->prev = r->current;
                    nr->speed = next_speed;
                    nr->elapsed = r->elapsed + (double)e->distance/next_speed;
                    bh_push(bh, nr);

                    if (r->prev == -1) break;
                }

                next_edge:; 
                e = e->next;
            }
        }

        if (fastest) {
            printf("%.5lf\n", fastest->elapsed);
        }
        else {
            printf("unreachable\n");
        }
        free(edge);
    }

    route_delete();
    bh_delete(bh);

    return 0;
}