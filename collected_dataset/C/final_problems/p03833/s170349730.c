#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
 
#define sl(x) scanf("%ld", &x)
#define ss(x) scanf("%s", x)
#define pl(x) printf("%ld\n", x)
#define ps(x) printf("%s\n", x)
 
#define INF 1000000000000000000
#define MOD 1000000007
 
#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))

long N, M;
long A[5001];
long B[5001][201];

long solve(long s) {
	long i, j;
	long max_B[201] = {0};
	long umai;
	long max_umai = -INF;
	long cost;
	
	for(i=s;i<=N;i++) {
		umai = 0;
		for(j=1;j<=M;j++) {
			max_B[j] = MAX(max_B[j], B[i][j]);
			umai += max_B[j];
		}
		cost = 0;
		for(j=s;j<i;j++) {
			cost += A[j];
		}
		max_umai = MAX(max_umai, umai-cost);
	}
	
	return max_umai;
}

int main() {
	long i, j;
	long ans = -INF;
	
	sl(N); sl(M);
	for(i=1;i<N;i++) {
		sl(A[i]);
	}
	for(i=1;i<=N;i++) {
		for(j=1;j<=M;j++) {
			sl(B[i][j]);
		}
	}
	
	for(i=1;i<=N;i+=2) {
		ans = MAX(ans, solve(i));
	}
	
	pl(ans);
	
	return 0;
}