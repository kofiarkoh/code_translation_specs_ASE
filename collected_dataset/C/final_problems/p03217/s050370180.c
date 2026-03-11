#include <stdio.h>
#include <stdlib.h>
#define inf (int)(2e9)
#define deque_valtype pair

typedef struct {
	int val;
	int num;
}pair;

typedef struct {
	deque_valtype *array;
	int first;
	int last;
}deque;

deque *make_deque(int max_size){
	deque *dq = (deque *)malloc(sizeof(deque));
	dq->array = (deque_valtype *)malloc(sizeof(deque_valtype) * max_size);
	dq->first = 0;
	dq->last = 0;
	return dq;
}

void add_last_data_deque(deque_valtype val, deque *dq){
	dq->array[dq->last] = val;
	dq->last++;
}

deque_valtype take_first_data_deque(deque *dq){
	dq->first++;
	return dq->array[dq->first - 1];
}

deque_valtype take_last_data_deque(deque *dq){
	dq->last--;
	return dq->array[dq->last];
}

deque_valtype look_first_data_deque(deque *dq){
	return dq->array[dq->first];
}

deque_valtype look_last_data_deque(deque *dq){
	return dq->array[dq->last - 1];
}

int element_num_deque(deque *dq){
	return dq->last - dq->first;
}

void flush_deque(deque *dq){
	dq->first = 0;
	dq->last = 0;
}

void flip(int **A, int N){
	int i, j, tmp;
	for(i = 0; i < N; i++){
		for(j = 0; j < i; j++){
			tmp = A[i][j];
			A[i][j] = A[j][i];
			A[j][i] = tmp;
		}
	}
}

void range_max(int *A, int *B, int N, int K, deque *dq){
	int i;
	if(K <= 0){
		for(i = 0; i < N; i++){
			B[i] = -inf;
		}
	}
	else{
		pair res;
		for(i = N - 1; i >= 0; i--){
			while(element_num_deque(dq) > 0 && look_last_data_deque(dq).val <= A[i]){
				res = take_last_data_deque(dq);
			}
			add_last_data_deque((pair){A[i], i}, dq);
			while(element_num_deque(dq) > 0 && look_first_data_deque(dq).num >= i + K){
				res = take_first_data_deque(dq);
			}
			B[i] = look_first_data_deque(dq).val;
		}
		flush_deque(dq);
	}
}

void out(int **A, int N){
	int i, j;
	for(i = 0; i < N; i++){
		for(j = 0; j < N; j++){
			printf("%d ", A[i][j]);
		}
		printf("\n");
	}
}

int select(int l, int r, int d, int D){
	d--;
	int h, s, a, b;
	h = (l + r) / 2;
	s = (h - d) / D;
	a = s * D + d;
	b = a + D;
	if(h - a < b - h){
		if(l < a){
			return a;
		}
		else{
			return h;
		}
	}
	else{
		if(b < r){
			return b;
		}
		else{
			return h;
		}
	}
}

signed main(){
	int N, D, x, y, i, j;
	scanf("%d%d", &N, &D);
	int **A = (int **)malloc(sizeof(int *) * 2 * D);
	int **subd = (int **)malloc(sizeof(int *) * 2 * D);
	int **sube = (int **)malloc(sizeof(int *) * 2 * D);
	int **subdd = (int **)malloc(sizeof(int *) * 2 * D);
	int **subde = (int **)malloc(sizeof(int *) * 2 * D);
	int **subed = (int **)malloc(sizeof(int *) * 2 * D);
	int **subee = (int **)malloc(sizeof(int *) * 2 * D);
	for(i = 0; i < 2 * D; i++){
		A[i] = (int *)malloc(sizeof(int) * 2 * D);
		subd[i] = (int *)malloc(sizeof(int) * 2 * D);
		sube[i] = (int *)malloc(sizeof(int) * 2 * D);
		subdd[i] = (int *)malloc(sizeof(int) * 2 * D);
		subde[i] = (int *)malloc(sizeof(int) * 2 * D);
		subed[i] = (int *)malloc(sizeof(int) * 2 * D);
		subee[i] = (int *)malloc(sizeof(int) * 2 * D);
		for(j = 0; j < 2 * D; j++){
			A[i][j] = 0;
		}
	}
	for(i = 0; i < N; i++){
		scanf("%d%d", &x, &y);
		A[x % D][y % D]++;
		A[x % D][D + y % D]++;
		A[D + x % D][y % D]++;
		A[D + x % D][D + y % D]++;
	}
/*	printf("A\n");
	out(A, 2 * D);
*/	int l = 0, h, r = 1000 * D;
	int k, d, e, f, oldd = -1;
	deque *dq = make_deque(2 * D);
	while(r - l > 1){
		h = select(l, r, oldd, D);
//		h = (l + r) / 2;
		k = h / D;
		d = h % D + 1;
		e = D - d;
//		printf("(h, k, d, e) = (%d, %d, %d, %d)\n", h, k, d, e);
		if(d != oldd){
		for(i = 0; i < 2 * D; i++){
			range_max(A[i], subd[i], 2 * D, d, dq);
			range_max(A[i], sube[i], 2 * D, e, dq);
		}

/*		printf("subd\n");
		out(subd, 2 * D);
		printf("sube\n");
		out(sube, 2 * D);
*/
		flip(subd, 2 * D);
		flip(sube, 2 * D);
		for(i = 0; i < 2 * D; i++){
			range_max(subd[i], subdd[i], 2 * D, d, dq);
			range_max(subd[i], subde[i], 2 * D, e, dq);
			range_max(sube[i], subed[i], 2 * D, d, dq);
			range_max(sube[i], subee[i], 2 * D, e, dq);
		}
/*		flip(subdd, 2 * D);
		flip(subde, 2 * D);
		flip(subed, 2 * D);
		flip(subee, 2 * D);
*/
/*		printf("subdd\n");
		out(subdd, 2 * D);
		printf("subde\n");
		out(subde, 2 * D);
		printf("subed\n");
		out(subed, 2 * D);
		printf("subee\n");
		out(subee, 2 * D);
*/		}
		f = 0;
		for(i = 0; i < D && f == 0; i++){
			for(j = 0; j < D; j++){
				if(subdd[j][i] <= (k + 1) * (k + 1) && subde[j][i + d] <= k * (k + 1) && subed[j + d][i] <= k * (k + 1) && subee[j + d][i + d] <= k * k){
					f = 1;
					break;
				}
			}
		}
//		printf("f = %lld\n", f);
		if(f == 1){
			r = h;
		}
		else{
			l = h;
		}
		oldd = d;
	}
	printf("%d\n", r);
	return 0;
}