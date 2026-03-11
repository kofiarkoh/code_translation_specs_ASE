#include <stdio.h>
#include <stdlib.h>

#define MAX_NODE (100000)

int parent[MAX_NODE] = {0};
int* tree_cache[MAX_NODE];
int t_size[MAX_NODE];

void init_tree(int root, int* leafs, int leaf_size) {
    int i,j,node,id;
    int cache[MAX_NODE];
    
    // 葉の要素だけ実体化する
    for(i=0;i<leaf_size;i++) {
        id = leafs[i];
        cache[0] = id;
        for(j=1;j<MAX_NODE;j++) {
            node = parent[cache[j-1]];
            cache[j] = node;
            if(node == root) {
                t_size[id] = j + 1;
                break;
            }
        }
        tree_cache[id] = (int*)malloc(t_size[id]*sizeof(int));
        for(j=0;j<t_size[id];j++) tree_cache[id][j] = cache[j];

        // 途中の要素はポインタを設定
        for(j=1;j<t_size[id];j++) {
            node = tree_cache[id][j];
            tree_cache[node] = &tree_cache[id][j];
            t_size[node] = t_size[id] - j;
        }
    }
}

int binary_search(int left, int right, int* so, int* si, int root) {
    int i, node;
    int middle = (left + right) / 2;
    if(right - left < 40) {
        for(i=left,node=root;i<=right;i++) {
            if(so[i] == si[i]) {
                return so[i];
            }
        }
        return node;
    }
    
    if(so[middle] != si[middle]) {
        return binary_search(middle, right, so, si, root);
    }
    
    return binary_search(left, middle, so, si, root);
}

int solve(int source, int sink, int root) {
    int node;
    int last;
    int* p_so_list;
    int* p_si_list;
    int so, si;
    if(source == sink) return source;
    so = si = 0;

    p_so_list = tree_cache[source];
    so = t_size[source];
    p_si_list = tree_cache[sink];
    si = t_size[sink];

    if(si > so) {
        last = si - so;
        return binary_search(0, so, p_so_list, &p_si_list[last], root);
    }
    
    last = so - si;
    return binary_search(0, si, &p_so_list[last], p_si_list, root);
    
}

int main(void) {
    int i, j, m, n, t, q;
    int source, sink;
    int is_root[MAX_NODE] = {0};
    int leafs[MAX_NODE];
    int leaf_size = 0;
    int root;

    scanf("%d\n", &n);
    for(i=0;i<n;i++) {
        scanf("%d", &m);
        if(m == 0) leafs[leaf_size++] = i;

        for(j=0;j<m;j++) {
            scanf("%d", &t);
            is_root[t] = -1;
            parent[t] = i;
        }
    }
    
    // ルート
    root = 0;
    for(i=0;i<n;i++) {
        t_size[i] = -1;
        if(is_root[i] == 0) {
            root = i;
            break;
        }
    }

    init_tree(root, leafs, leaf_size);
    
    scanf("%d\n", &q);
    for(i=0;i<q;i++) {
        scanf("%d %d\n" ,&source, &sink);
        printf("%d\n", solve(source, sink, root));
    }

    return 0;
}