#include <stdio.h>
#include <stdlib.h>
#define int long long

signed compare(const void *a, const void *b){
	int sub = *(int *)a - *(int *)b;
	if(sub > 0){
		return -1;
	}
	else if(sub == 0){
		return 0;
	}
	else{
		return 1;
	}
}

int max(int a, int b){
	return a >= b ? a : b;
}

signed main(){
	int N, K, i, j, k;
	scanf("%lld%lld", &N, &K);
	int *Cnum = (int *)malloc(sizeof(int) * K);
	for(i = 0; i < K; i++){
		Cnum[i] = 0;
	}
	int *C = (int *)malloc(sizeof(int) * N);
	int *V = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%lld%lld", &C[i], &V[i]);
		C[i]--;
		Cnum[C[i]]++;
	}
	int **VofC = (int **)malloc(sizeof(int *) * K);
	for(i = 0; i < K; i++){
		VofC[i] = (int *)malloc(sizeof(int) * Cnum[i]);
		Cnum[i] = 0;
	}
	for(i = 0; i < N; i++){
		VofC[C[i]][Cnum[C[i]]] = V[i];
		Cnum[C[i]]++;
	}
	for(i = 0; i < K; i++){
		qsort(VofC[i], Cnum[i], sizeof(int), compare);
	}
	int **Vs = (int **)malloc(sizeof(int *) * K);
	for(i = 0; i < K; i++){
		Vs[i] = (int *)malloc(sizeof(int) * (Cnum[i] + 1));
		Vs[i][0] = 0;
		for(j = 1; j <= Cnum[i]; j++){
			Vs[i][j] = Vs[i][j - 1] + VofC[i][j - 1];
		}
	}
	int *old = (int *)malloc(sizeof(int) * (N + 1));
	int *new = (int *)malloc(sizeof(int) * (N + 1));
	for(i = 0; i <= Cnum[0]; i++){
		old[i] = Vs[0][i];
	}
	old[1] = -1;
	int now = Cnum[0];
	for(i = 1; i < K; i++){
		for(j = 0; j <= now + Cnum[i]; j++){
			new[j] = -1;
		}
//		printf("i = %lld\n", i);
/*		printf("old:\n");
		for(j = 0; j <= now; j++){
			printf("%lld ", old[j]);
		}
		printf("\n");
*/		for(j = 0; j <= now; j++){
			for(k = 0; k <= Cnum[i]; k++){
				if(old[j] >= 0 && k != 1){
					new[j + k] = max(new[j + k], old[j] + Vs[i][k]);
				}
			}
		}
		now += Cnum[i];
		for(j = 0; j <= now; j++){
			old[j] = new[j];
		}
	}
	for(i = 1; i <= N; i++){
		printf("%lld\n", old[i]);
	}
	return 0;
}