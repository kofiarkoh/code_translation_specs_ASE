#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
 
#define sl(x) scanf("%ld", &x)
#define ss(x) scanf("%s", x)
#define pl(x) printf("%ld\n", x)
#define pd(x) printf("%lf\n", x)
#define ps(x) printf("%s\n", x)
 
#define INF 1000000000000000000
#define MOD 1000000007
 
#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))
 
long N;
long map[501][501];
long ans = INF;
 
int compare_long(const void *a, const void *b) {
	return *(long*)a - *(long*)b;
}

int main() {
	long i, j, k, x, y, m;
	long e = 1;
	long empty[300001];
	long pre = 0;
	long can[300001] = {0};
	long sum = 0;
	char buf[510];
	long work[501][501];
	long cnt;
	long c;
	long C[501];
	
	sl(N);
	cnt = N * N * N;
	c = 1;
	for(i=1;i<=N;i++) {
		ss(buf);
		for(j=1;j<=N;j++) {
			map[i][j] = buf[j-1] == '#' ? 1 : 0;
			if(map[i][j] == 0) c = 0;
		}
	}
	
	if(c) {
		pl(0);
		return 0;
	}
	
	for(k=0;k<1000000/(N*N);k++) {
		for(i=1;i<=N;i++) for(j=1;j<=N;j++) work[i][j] = map[i][j];
		for(m=1;m<=cnt;m++) {
			i = rand() % N + 1; j = rand() % N + 1;
			for(x=1;x<=N;x++) C[x] = work[i][x];
			for(x=1;x<=N;x++) work[x][j] = C[x];
			c = 1;
			for(y=1;y<=N;y++) for(x=1;x<=N;x++) if(work[y][x] == 0) {c = 0; break;}
			if(c) {
				ans = MIN(ans, m);
			}
		}
	}
	
	pl(ans == INF ? -1 : ans);
	
	return 0;
}