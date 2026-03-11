#include <stdio.h>
#include <stdlib.h>
#define int long long

int p;

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

int power(int a, int N){
	if(N == 0){
		return 1;
	}
	else if(N % 2 == 0){
		return power(MOD(a * a), N / 2);
	}
	else{
		return MOD(a * power(a, N - 1));
	}
}

int ABS(int a){
	return a >= 0 ? a : -a;
}

int gcd(int a, int b){
	a = ABS(a);
	b = ABS(b);
	if(b == 0){
		return a;
	}
	else{
		return gcd(b, a % b);
	}
}

signed compair(const void *a, const void *b){
	int sub = *(int *)a - *(int *)b;
	if(sub < 0){
		return -1;
	}
	else if(sub == 0){
		return 0;
	}
	else{
		return 1;
	}
}

int min(int a, int b){
	return a <= b ? a : b;
}

signed main(){
	int N, i, C = 0, a0, j, k, l, m;
	scanf("%lld", &N);
	int *a = (int *)malloc(sizeof(int) * (N + 1));
	for(i = N; i >= 0; i--){
		scanf("%lld", &a[i]);
		C = gcd(C, a[i]);
	}
	for(i = 0; i <= N; i++){
		a[i] /= C;
	}
/*	int *is_prime = (int *)malloc(sizeof(int) * (M + 1));
	for(i = 0; i <= M; i++){
		is_prime[i] = 1;
	}
	is_prime[0] = 0;
	is_prime[1] = 0;
	for(i = 2; i <= M; i++){
		if(is_prime[i] == 1){
			for(j = 2; i * j <= M; j++){
				is_prime[i * j] = 0;
			}
		}
	}
*/	int *ans = (int *)malloc(sizeof(int) * 1000000);
	k = 0;
	for(i = 2; i * i <= C; i++){
		if(C % i == 0){
			ans[k] = i;
			k++;
			while(C % i == 0){
				C /= i;
			}
		}
	}
	if(C > 1){
		ans[k] = C;
		k++;
	}
/*	for(i = 0; i < k; i++){
		printf("ans[%lld] = %lld\n", i, ans[i]);
	}
*/
	int *d = (int *)malloc(sizeof(int) * 1000000);
	l = 0;
	a0 = a[0];
	for(i = 2; i * i <= a0; i++){
		if(a0 % i == 0){
			d[l] = i;
			l++;
			while(a0 % i == 0){
				a0 /= i;
			}
		}
	}
	if(a0 > 1){
		d[l] = a0;
		l++;
	}
/*	for(i = 0; i < l; i++){
		printf("d[%lld] = %lld\n", i, d[i]);
	}
*/	int x, val, f;
	for(i = 0; i < l; i++){
		f = 1;
		for(j = 0; j < min(N, d[i]); j++){
			x = j;
			p = d[i];
			val = 0;
			for(m = 0; m <= N; m++){
				val = MOD(val + a[m] * power(x, m));
			}
			if(val != 0){
				f = 0;
				break;
			}
		}
		if(f == 1){
			ans[k] = d[i];
			k++;
		}
	}



	qsort(ans, k, sizeof(int), compair);
	if(k == 0){
		printf("\n");
		return 0;
	}
	printf("%lld\n", ans[0]);
	for(i = 1; i < k; i++){
		if(ans[i] != ans[i - 1]){
			printf("%lld\n", ans[i]);
		}
	}
	return 0;
}