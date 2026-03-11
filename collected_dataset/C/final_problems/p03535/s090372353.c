#include <stdio.h>
#include <stdlib.h>
#define ms_valtype int

//比較関数
int compare_ms(ms_valtype a, ms_valtype b){
	return a - b;
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

int min(int a, int b){
	return a <= b ? a : b;
}

int main(){
	int N, i, m, inf, sup;
	scanf("%d", &N);
	int *D = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &D[i]);
	}
	sort(D, N);
/*	for(m = 0; ; m++){
		inf = m;
		sup = 100;
		for(i = 0; i < N; i++){
			if(D[i] - inf < sup - (24 - D[i])){
				if(D[i] - inf >= 0){

				}
			}
		}
	}
*/	for(i = 0; i < N; i++){
		if(i % 2 == 0){
			D[i] = 24 - D[i];
		}
	}
	sort(D, N);
	m = min(D[0] - 0, 24 - D[N - 1]);
	for(i = 1; i < N; i++){
		m = min(m, D[i] - D[i - 1]);
	}
	printf("%d\n", m);
	return 0;
}