#include <stdio.h>
#include <stdlib.h>

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
		uf->u[i] = i;
		uf->u_rank[i] = 1;
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

void combine_uf(int x, int y, union_find *uf){
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
	}
	else{
		u[y_root] = x_root;
		u_rank[x_root] += u_rank[y_root];
		u_rank[y_root] = 0;
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
	return uf->u_rank[root_uf(x, uf)];
}

int main(){
	int N, M, K, L, r, i, j;
	scanf("%d%d", &N, &M);
	union_find *uf = make_union_find(N + M);
	for(i = 0; i < N; i++){
		scanf("%d", &K);
		for(j = 0; j < K; j++){
			scanf("%d", &L);
			L--;
			combine_uf(i, N + L, uf);
		}
	}
	r = root_uf(0, uf);
	for(i = 0; i < N; i++){
		if(root_uf(i, uf) != r){
			printf("NO\n");
			return 0;
		}
	}
	printf("YES\n");
	return 0;
}