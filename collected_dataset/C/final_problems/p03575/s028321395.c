#include <stdio.h>
#include <string.h>
#include <stdlib.h>

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

#define VERTEX_MAX 50
#define EDGE_MAX ((VERTEX_MAX*(VERTEX_MAX-1))/2)

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

void init_stack(void) {
    pos = 0;
}

struct pair {
    int v1;
    int v2;
};

int main(void) {
    int vs, es;
    get_int2(&vs, &es);
    int i, j;
    struct pair ps[EDGE_MAX];
    static char map[VERTEX_MAX][VERTEX_MAX];
    for(i = 0; i < es; i++) {
        int v1, v2;
        get_int2(&v1, &v2);
        // 1-indexed -> 0-indexed
        v1--; v2--;
        ps[i].v1 = v1; ps[i].v2 = v2;
        map[v1][v2] = map[v2][v1] = 1;
    }

    int ans = 0;
    for(i = 0; i < vs; i++) {
        int v1 = ps[i].v1; int v2 = ps[i].v2;
        char check[VERTEX_MAX] = {0};
        init_stack();
        push(v1);
        check[v1] = 1;
        // dfs
        while(!is_empty()) {
            int v = peek();
            int nv;
            int flag = 0;
            for(nv = 0; nv < vs; nv++) {
                // exclude edge
                if(v == v1 && nv == v2) continue;
                if(nv == v1 && v == v2) continue;
                if(map[v][nv] && !check[nv]) {
                    check[nv] = 1;
                    push(nv);
                    flag = 1;
                    break;
                }
            }
            if(!flag) pop();
        }
        int flag = 1;
        for(j = 0; j < vs; j++) {
            if(!check[j]) {
                flag = 0;
                break;
            }
        }
        // if separated
        if(!flag) ans++;
    }

    printf("%d\n", ans);
    return 0;
}