#include <stdio.h>
#include <stdlib.h>
#define int long long
#define inv_num_valtype int

//比較関数
int compare(inv_num_valtype a, inv_num_valtype b){
	return a - b;
}

//昇順
int inv_num_sub(inv_num_valtype *origin, int left, int right, inv_num_valtype *tmp){
	int ans = 0;
	if(right - left > 1){
		int i, j, k, half = (left + right) / 2;
		ans += inv_num_sub(origin, left, half, tmp);
		ans += inv_num_sub(origin, half, right, tmp);
		for(i = left; i < right; i++){
			tmp[i] = origin[i];
		}
		for(i = left, j = left, k = half; i < right; i++){
			if(k == right){
				origin[i] = tmp[j];
				j++;
			}
			else if(compare(tmp[j], tmp[k]) <= 0 && j < half){
				origin[i] = tmp[j];
				j++;
			}
			else{
				origin[i] = tmp[k];
				ans += k - i;
				k++;
			}
		}
	}
	return ans;
}

//i < j && a[i] > a[j] となる組(i, j)の数
//effect: sortされる
int inv_num(inv_num_valtype *origin, int N){
	inv_num_valtype *tmp = (inv_num_valtype *)malloc(sizeof(inv_num_valtype) * N);
	return inv_num_sub(origin, 0, N, tmp);
}

int less_than(int k, int N, int *a, int *b){
	int i;
	b[0] = 0;
	for(i = 0; i < N; i++){
		if(a[i] > k){
			b[i + 1] = b[i] + 1;
		}
		else{
			b[i + 1] = b[i] - 1;
		}
	}
	return inv_num(b, N + 1);
}

signed main(){
	int N, i;
	scanf("%lld", &N);
	int *a = (int *)malloc(sizeof(int) * N);
	int *b = (int *)malloc(sizeof(int) * (N + 1));
	int *c = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%lld", &a[i]);
		c[i] = a[i];
	}
	int res = inv_num(c, N);
	int l = -1, h, r = N;
	while(r - l > 1){
		h = (l + r) / 2;
		if(less_than(c[h], N, a, b) > ((N * (N + 1)) / 2) / 2){
			r = h;
		}
		else{
			l = h;
		}
	}
	printf("%lld\n", c[r]);
	return 0;
}