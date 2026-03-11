#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

//for
#define REP(i,n) for(int i=0,i##_len=(int)(n);i<i##_len;i++)
#define REPS(i,n) for(int i=1,i##_len=(int)(n);i<=i##_len;i++)
#define RREP(i,n) for(int i=(int)(n)-1;i>=0;i--)
#define RREPS(i,n) for(int i=(int)(n);i>0;i--)

//printf
#define PRINTD(d) printf("%d\n",(d))
#define PRINTS(s) printf("%s\n",(s))

//memset
#define m0(s) memset(s,0,sizeof(s))
#define ml(s) memset(s,63,sizeof(s))
#define fill(s,c) memset(s,c,sizeof(s))

typedef long long ll;

int Min(int a, int b) {
	return (a)<(b) ? (a) : (b);
}
int Max(int a, int b) {
	return (a) > (b) ? (a) : (b);
}

int a[100000];
int g[100000];
int m[100000];
int f[100000] = { 0 };

int dfs(int i) {
	if (g[i] != i) {
		return g[i] = dfs(g[i]);
	}
	else {
		return i;
	}
}

int main() {
	int N, M;
	scanf("%d %d", &N, &M);
	REP(i, N) g[i] = i;
	REP(i, N) m[i] = 1000000001;
	REP(i, N) scanf("%d", &a[i]);
	//REP(i, N) a[i] = i;
	REP(i, M) {
		int x,y;
		scanf("%d %d", &x, &y);
		//x = i; y = i + N / 2;
		f[x] = f[y] = 1;
		int g1 = dfs(x);
		int g2 = dfs(y);
		if (a[g1] <= a[g2]) {
			g[g2] = g1;
			m[g2] = 0;
			if (a[g2] < m[g1])m[g1] = a[g2];
		}
		else {
			g[g1] = g2;
			m[g1] = 0;
			if (a[g1] < m[g2])m[g2] = a[g1];
		}
	}
	int cnt = 0;
	REP(i, N) {
		if (f[i]==0)cnt++;
	}
	if (cnt > 2) {
		PRINTS("Impossible");
		return 0;
	}
	ll sum = 0;
	int max[2] = {0};
	int flag = 0;
	REP(i, N) {
		if (g[i] == i) {
			flag++;
			sum += a[i];
			//printf("%d sum=%lld\n", i, sum);
			if (m[i] <= 1000000000) {
				REP(j, 2) {
					if (max[j] < m[i]) {
						sum += max[1];
						if (j == 0) {
							max[1] = max[0];
						}
						max[j] = m[i];
						//printf("max=%d %d\n", max[0], max[1]);
						break;
					}
				}
			}
		}
	}
	/*
	REP(i, N) {
		printf("%d ", g[i]);
	}
	PRINTS("");
	REP(i, N) {
		printf("%d ", m[i]);
	}
	PRINTS("");
	*/
	if (flag==1) {
		PRINTD(0);
	}
	else {
		printf("%lld\n", sum);
	}
}