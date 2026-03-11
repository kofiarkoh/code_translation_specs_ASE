#include <stdio.h>
#include <stdlib.h>

#define int long long

int min(int a, int b){
	if(a < b){
		return a;
	}
	else{
		return b;
	}
}

int MOD(int a, int m){
	if(a >= 0){
		return a % m;
	}
	else{
		return m + a % m;
	}
}

signed main(){
	int n, m, i;
	scanf("%lld%lld", &n, &m);
	int *a = (int *)malloc(sizeof(int) * n);
	for(i = 0; i < n; i++){
		scanf("%lld", &a[i]);
	}
	int *l = (int *)malloc(sizeof(int) * m);
	for(i = 0; i < m; i++){
		l[i] = 0;
	}
	int cont = 0;
	for(i = 1; i < n; i++){
		if(MOD(a[i], m) < MOD(a[i - 1], m)){
			cont++;
		}
	}
	for(i = 0; i < m; i++){
		l[i] += cont;
	}
	for(i = (a[0] + 2) % m; i != (a[n - 1] + 2) % m; i = (i + 1) % m){
		l[i]++;
	}
	/*for(i = 0; i < m; i++){
		printf("%lld  ", l[i]);
	}
	printf("\n");*/
	for(i = 1; i < n; i++){
		l[MOD(a[i] + 1, m)] -= MOD(a[i] - a[i - 1], m);
	}
	/*for(i = 0; i < m; i++){
		printf("%lld  ", l[i]);
	}
	printf("\n");*/
	for(i = 1; i < m; i++){
		l[i] += l[i - 1];
	}
	/*for(i = 0; i < m; i++){
		printf("%lld  ", l[i]);
	}
	printf("\n");*/
	int x = 0, mm = l[0];
	for(i = 1; i < m; i++){
		if(mm < l[i]){
			mm = l[i];
			x = i;
		}
	}
	int ans = 0;
	for(i = 1; i < n; i++){
		ans += min(MOD(a[i] - a[i - 1], m), MOD(a[i] - x + 1, m));
	}
	printf("%lld\n", ans);
}