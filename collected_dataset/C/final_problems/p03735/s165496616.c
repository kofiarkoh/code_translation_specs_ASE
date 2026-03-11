#include <stdio.h>
#define inf (long long int)(1e18 + 1)

long long int max(long long int a, long long int b){
	if(a > b){
		return a;
	}
	else{
		return b;
	}
}

long long int min(long long int a, long long int b){
	if(a < b){
		return a;
	}
	else{
		return b;
	}
}

long long int max4(long long int a, long long int b, long long int c, long long int d){
	return max(max(a, b), max(c, d));
}

long long int min4(long long int a, long long int b, long long int c, long long int d){
	return min(min(a, b), min(c, d));
}

int main(){
	long long int N, i, j, k, l, tmp, x, y;
	scanf("%lld", &N);
	long long int cand[4][2];
	cand[0][0] = 0;
	cand[1][0] = inf;
	cand[2][1] = 0;
	cand[3][1] = inf;
	for(i = 0; i < N; i++){
		scanf("%lld%lld", &x, &y);
		if(x < y){
			tmp = x;
			x = y;
			y = tmp;
		}
		if(x > cand[0][0]){
			cand[0][0] = x;
			cand[0][1] = y;
		}
		if(x < cand[1][0]){
			cand[1][0] = x;
			cand[1][1] = y;
		}
		if(y > cand[2][1]){
			cand[2][0] = x;
			cand[2][1] = y;
		}
		if(y < cand[3][1]){
			cand[3][0] = x;
			cand[3][1] = y;
		}
	}
	long long int ans = inf;
	for(i = 0; i < 2; i++){
		for(j = 0; j < 2; j++){
			for(k = 0; k < 2; k++){
				for(l = 0; l < 2; l++){
					ans = min((max4(cand[0][i], cand[1][j], cand[2][k], cand[3][l]) 
						- min4(cand[0][i], cand[1][j], cand[2][k], cand[3][l])) 
					* (max4(cand[0][1 - i], cand[1][1 - j], cand[2][1 - k], cand[3][1 - l]) 
						- min4(cand[0][1 - i], cand[1][1 - j], cand[2][1 - k], cand[3][1 - l])), ans);
				}
			}
		}
	}
	printf("%lld\n", ans);
	return 0;
}