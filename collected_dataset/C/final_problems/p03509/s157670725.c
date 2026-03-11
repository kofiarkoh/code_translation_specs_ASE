#include <stdio.h>
#include <stdlib.h>
#define int long long
#define ms_valtype pair

typedef struct {
	int w;
	int b;
}pair;

//比較関数
int compare_ms(ms_valtype a, ms_valtype b){
	return (b.b + b.w) - (a.b + a.w);
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

signed main(){
	int N, P, i, sum = 0;
	scanf("%lld%lld", &N, &P);
	pair *ps = (pair *)malloc(sizeof(pair) * N);
	for(i = 0; i < N; i++){
		scanf("%lld%lld", &ps[i].w, &ps[i].b);
		ps[i].w *= (100 - P);
		ps[i].b *= P;
		sum -= ps[i].b;
	}
	sort(ps, N);
	for(i = 0; i < N; i++){
		sum += ps[i].w + ps[i].b;
		if(sum >= 0){
			printf("%lld\n", i + 1);
			return 0;
		}
	}
}