#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define inf 1e18

typedef struct {
	int u;
	int v;
	double d;
}edge;

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

void flush_uf(union_find *uf){
	int i;
	for(i = 0; i < uf->N; i++){
		uf->u[i] = i;
		uf->u_rank[i] = 1;
	}
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
	return (uf->u_rank)[root_uf(x, uf)];
}

int compare(const void *a, const void *b){
	if(((edge *)a)->d - ((edge *)b)->d < 0){
		return -1;
	}
	else if(((edge *)a)->d - ((edge *)b)->d > 0){
		return 1;
	}
	else{
		return 0;
	}
}

double max(double a, double b){
	return a >= b ? a : b;
}

double min(double a, double b){
	return a <= b ? a : b;
}

int main(){
	int N, i, j, k, l;
	scanf("%d", &N);
	double *x = (double *)malloc(sizeof(double) * N);
	double *y = (double *)malloc(sizeof(double) * N);
	double *a = (double *)malloc(sizeof(double) * N);
	for(i = 0; i < N; i++){
		scanf("%lf%lf%lf", &x[i], &y[i], &a[i]);
	}
	edge *e = (edge *)malloc(sizeof(edge) * ((N * (N - 1)) / 2));
	double *amaxmin = (double *)malloc(sizeof(double) * (1 << N));
	union_find *uf = make_union_find(N);
	double mst, asum;
	int V;
	amaxmin[0] = inf;
	for(i = 1; i < (1 << N); i++){
//		printf("i = %d\n", i);
		mst = 0;
		asum = 0;
		V = 0;
		l = 0;
		for(j = 0; j < N; j++){
			if((i | (1 << j)) == i){
				for(k = j + 1; k < N; k++){
					if((i | (1 << k)) == i){
						e[l].u = j;
						e[l].v = k;
						e[l].d = hypot(x[j] - x[k], y[j] - y[k]);
						l++;
					}
				}
				asum += a[j];
				V++;
			}
		}
		qsort(e, l, sizeof(edge), compare);
//		printf("l = %d\n", l);
		flush_uf(uf);
		for(j = 0; j < l; j++){
			if(is_same_union_uf(e[j].u, e[j].v, uf) == 0){
				combine_uf(e[j].u, e[j].v, uf);
				mst += e[j].d;
			}
		}
		amaxmin[i] = (asum - mst) / V;
//		printf("(mst, asum, V, amaxmin) = (%lf, %lf, %d, %lf)\n", mst, asum, V, amaxmin[i]);
	}
	double *dp = (double *)malloc(sizeof(double) * (1 << N));
	dp[0] = inf;
	for(i = 1; i < (1 << N); i++){
		dp[i] = amaxmin[i];
		for(j = (i & (i - 1)); j > 0; j = (i & (j - 1))){
			dp[i] = max(dp[i], min(dp[j], dp[i ^ j]));
		}
	}
	printf("%.12lf\n", dp[(1 << N) - 1]);
	return 0;
}