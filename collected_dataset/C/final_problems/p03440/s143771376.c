#include <stdio.h>
#include <stdlib.h>
#define int long long
#define heap_valtype int

int *A;

int compare(heap_valtype a, heap_valtype b){
	return A[a] - A[b];
}

typedef struct heap_node_sub{
	heap_valtype val;
	struct heap_node_sub *parent;
	struct heap_node_sub *prev;
	struct heap_node_sub *next;
	struct heap_node_sub *left;
	struct heap_node_sub *right;
}heap_node;

typedef struct {
	int N;
	heap_node *root;
	heap_node *last;
}heap;

//heapを生成
heap *make_heap(){
	heap *h = (heap *)malloc(sizeof(heap));
	h->N = 0;
	h->root = NULL;
	h->last = NULL;
	return h;
}

int element_num_heap(heap *h){
	return h->N;
}

void up_heapify(heap_node *n, heap *h){
	if(n->parent != NULL){
		if(compare(n->val, n->parent->val) < 0){
			heap_valtype tmp;
			tmp = n->val;
			n->val = n->parent->val;
			n->parent->val = tmp;
			up_heapify(n->parent, h);
		}
	}
}

//データを追加
void add_data_heap(heap_valtype val, heap *h){
	heap_node *new = (heap_node *)malloc(sizeof(heap_node));
	new->val = val;
	new->next = NULL;
	new->left = NULL;
	new->right = NULL;
	if(element_num_heap(h) == 0){
		new->parent = NULL;
		new->prev = NULL;
		h->root = new;
		h->last = new;
	}
	else if(element_num_heap(h) == 1){
		new->parent = h->root;
		new->prev = h->root;
		h->root->left = new;
		h->root->next = new;
		h->last = new;
		up_heapify(new, h);
	}
	else{
		new->prev = h->last;
		h->last->next = new;
		if(h->last->parent->right == NULL){
			new->parent = h->last->parent;
			h->last->parent->right = new;
		}
		else{
			new->parent = h->last->parent->next;
			h->last->parent->next->left = new;
		}
		h->last = new;
		up_heapify(new, h);
	}
	h->N++;
}

void down_heappify(heap_node *n, heap *h){
	if(n->left != NULL){
		heap_valtype tmp;
		if(n->right == NULL){
			if(compare(n->val, n->left->val) > 0){
				tmp = n->val;
				n->val = n->left->val;
				n->left->val = tmp;
				down_heappify(n->left, h);
			}
		}
		else{
			if(compare(n->left->val, n->right->val) <= 0 && compare(n->val, n->left->val) > 0){
				tmp = n->val;
				n->val = n->left->val;
				n->left->val = tmp;
				down_heappify(n->left, h);
			}
			else if(compare(n->left->val, n->right->val) > 0 && compare(n->val, n->right->val) > 0){
				tmp = n->val;
				n->val = n->right->val;
				n->right->val = tmp;
				down_heappify(n->right, h);
			}
		}
	}
}

//最小のものを取り出す
heap_valtype take_min_heap(heap *h){
	if(element_num_heap(h) == 0){
		printf("no data in the heap\n");
	}
	heap_valtype ans = h->root->val;
	heap_node *ln = h->last;
	if(element_num_heap(h) == 1){
		h->root = NULL;
		h->last = NULL;
	}
	else{
		ln->prev->next = NULL;
		if(ln->parent->right == NULL){
			ln->parent->left = NULL;
		}
		else{
			ln->parent->right = NULL;
		}
		h->last = ln->prev;
		h->root->val = ln->val;
		down_heappify(h->root, h);
	}
	free(ln);
	h->N--;
	return ans;
}

heap_valtype look_min_heap(heap *h){
	if(element_num_heap(h) == 0){
		printf("no data in the heap\n");
	}
	return h->root->val;
}

typedef struct {
	int N;
	int *u;
	int *u_rank;
}union_find;

union_find *make_union_find(int N){
	int i;
	union_find *uf = (union_find *)malloc(sizeof(union_find));
	uf->N = N;
	uf->u = (int *)malloc(sizeof(int) * N);
	uf->u_rank = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		(uf->u)[i] = i;
		(uf->u_rank)[i] = 1;
	}
	return uf;
}

int root_uf(int x, union_find *uf){
	int *u = uf->u;
	if(u[x] == x){
		return x;
	}
	else{
		u[x] = root_uf(u[x], uf);
		return u[x];
	}
}

void combine_uf(int x, int y, union_find *uf, heap **hs){
	int x_root = root_uf(x, uf);
	int y_root = root_uf(y, uf);
	int *u = uf->u;
	int *u_rank = uf->u_rank;
	if(x_root == y_root){
		return;
	}
	else if(u_rank[x_root] < u_rank[y_root]){
		u[x_root] = y_root;
		u_rank[y_root] += u_rank[x_root];
		u_rank[x_root] = 0;
		while(element_num_heap(hs[x_root]) > 0){
			add_data_heap(take_min_heap(hs[x_root]), hs[y_root]);
		}
	}
	else{
		u[y_root] = x_root;
		u_rank[x_root] += u_rank[y_root];
		u_rank[y_root] = 0;
		while(element_num_heap(hs[y_root]) > 0){
			add_data_heap(take_min_heap(hs[y_root]), hs[x_root]);
		}
	}
}

//xとyが同じ集合に属していれば1を,そうでなければ0を返す
int is_same_union_uf(int x, int y, union_find *uf){
	if(root_uf(x, uf) == root_uf(y, uf)){
		return 1;
	}
	else{
		return 0;
	}
}

//xが属する集合の要素数を返す
int rank_uf(int x, union_find *uf){
	return (uf->u_rank)[root_uf(x, uf)];
}

signed main(){
	int N, M, x, y, i;
	scanf("%lld%lld", &N, &M);
	A = (int *)malloc(sizeof(int) * N);
	union_find *uf = make_union_find(N);
	for(i = 0; i < N; i++){
		scanf("%lld", &A[i]);
	}
	heap **hs = (heap **)malloc(sizeof(heap *) * N);
	for(i = 0; i < N; i++){
		hs[i] = make_heap();
		add_data_heap(i, hs[i]);
	}
	for(i = 0; i < M; i++){
		scanf("%lld%lld", &x, &y);
		combine_uf(x, y, uf, hs);
	}
	heap *H_only1 = make_heap();
	heap *H_over2 = make_heap();
	for(i = 0; i < N; i++){
		if(i == root_uf(i, uf)){
			if(element_num_heap(hs[i]) == 1){
				add_data_heap(look_min_heap(hs[i]), H_only1);
			}
			else{
				add_data_heap(look_min_heap(hs[i]), H_over2);
			}
		}
	}
	int val1, val2, val11, val12, val21, val22, ans = 0;
	while(1){
		if(element_num_heap(H_only1) + element_num_heap(H_over2) == 1){
			printf("%lld\n", ans);
			return 0;
		}
		if(element_num_heap(H_over2) == 0){
			if(element_num_heap(H_only1) > 2){
				printf("Impossible\n");
				return 0;
			}
			else{
				printf("%lld\n", ans + A[take_min_heap(H_only1)] + A[take_min_heap(H_only1)]);
				return 0;
			}
		}
		else{
			if(element_num_heap(H_over2) == 1){
				val1 = take_min_heap(H_only1);
				val2 = take_min_heap(H_over2);
			}
			else{
				val21 = take_min_heap(H_over2);
				val22 = take_min_heap(H_over2);
				if(element_num_heap(H_only1) == 0){
					val1 = val21;
					val2 = val22;
				}
				else{
					val11 = take_min_heap(H_only1);
					if(A[val11] <= A[val22]){
						val1 = val11;
						val2 = val21;
						add_data_heap(val22, H_over2);
					}
					else{
						val1 = val21;
						val2 = val22;
						add_data_heap(val11, H_only1);
					}
				}
			}
			ans += A[take_min_heap(hs[root_uf(val1, uf)])] + A[take_min_heap(hs[root_uf(val2, uf)])];
			combine_uf(val1, val2, uf, hs);
			if(element_num_heap(hs[root_uf(val1, uf)]) == 1){
				add_data_heap(look_min_heap(hs[root_uf(val1, uf)]), H_only1);
			}
			else{
				add_data_heap(look_min_heap(hs[root_uf(val1, uf)]), H_over2);
			}
//			printf("ans = %lld\n", ans);
		}
	}
}