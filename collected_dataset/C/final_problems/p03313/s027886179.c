#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int num;
	int val;
}pair;

#define ms_valtype pair

//比較関数
int compare_ms(ms_valtype a, ms_valtype b){
	return a.val - b.val;
}

//昇順
void sort_sub(ms_valtype *origin, int left, int right, ms_valtype *tmp){
	if(right - left > 1){
		int i, j, k, half = (left + right) / 2;
		sort_sub(origin, left, half, tmp);
		sort_sub(origin, half, right, tmp);
		for(i = left; i < right; i++){
			tmp[i] = origin[i];
		}
		for(i = left, j = left, k = half; i < right; i++){
			if(k == right){
				origin[i] = tmp[j];
				j++;
			}
			else if(compare_ms(tmp[j], tmp[k]) <= 0 && j < half){
				origin[i] = tmp[j];
				j++;
			}
			else{
				origin[i] = tmp[k];
				k++;
			}
		}
	}
}

void sort(ms_valtype *origin, int N){
	ms_valtype *tmp = (ms_valtype *)malloc(sizeof(ms_valtype) * N);
	sort_sub(origin, 0, N, tmp);
	free(tmp);
}

int max(int a, int b){
	return a >= b ? a : b;
}

int main(){
	int N, i, j, k, c;
	scanf("%d", &N);
	pair *A = (pair *)malloc(sizeof(pair) * (1 << N));
	for(i = 0; i < (1 << N); i++){
		scanf("%d", &A[i].val);
		A[i].num = i;
	}
	sort(A, (1 << N));
	int *first = (int *)malloc(sizeof(int) * (1 << N));
	int *second = (int *)malloc(sizeof(int) * (1 << N));
	for(i = 0; i < (1 << N); i++){
		first[i] = 0;
		second[i] = 0;
	}
	for(i = (1 << N) - 1; i >= 0; i--){
//		printf("i = %d\n", i);
//		printf("A = (%d, %d)\n", A[i].num, A[i].val);
		if(second[A[i].num] > 0){
			continue;
		}
		else{
			c = (((1 << N) - 1) ^ A[i].num);
			j = c;
			do{
//				printf("j = %d\n", j);
				k = (A[i].num ^ j);
				if(first[k] == 0){
					first[k] = A[i].val;
				}
				else if(second[k] == 0){
					second[k] = A[i].val;
				}
				j = (c & (j - 1));
			}while(j != c);
		}
/*		printf("first:\n");
		for(j = 0; j < (1 << N); j++){
			printf("%d ", first[j]);
		}
		printf("\n");
		printf("second:\n");
		for(j = 0; j < (1 << N); j++){
			printf("%d ", second[j]);
		}
		printf("\n");
*/	}
	int ans = 0;
	for(i = 1; i < (1 << N); i++){
		ans = max(ans, first[i] + second[i]);
		printf("%d\n", ans);
	}
	return 0;
}