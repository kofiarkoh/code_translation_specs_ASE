#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
 
#define INF 10000000000
#define M 1000000007

#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

int compare_long(const void *a, const void *b) {
	return *(long*)a - *(long*)b;
}

int main() {
	long W, H;
	long p[100001];
	long q[100001];
	long i, j;
	long ans = 0;
	int wc = 0, hc = 0;
	
	scanf("%ld %ld", &W, &H);
	for(i=0;i<W;i++) {
		scanf("%ld", &p[i]);
	}
	p[W] = INF;
	qsort((void *)p, W, sizeof(long), compare_long);
	
	for(i=0;i<H;i++) {
		scanf("%ld", &q[i]);
	}
	q[H] = INF;
	qsort((void *)q, H, sizeof(long), compare_long);
	
	for(i=0;i<W+H;i++) {
		//printf("%d %d -> ", wc, hc);
		if(p[wc] < q[hc]) {
			ans += (p[wc] * (H + 1 - hc));
			wc++;
		} else {
			ans += (q[hc] * (W + 1 - wc));
			hc++;
		}
		//printf("%d %d: %ld\n", wc, hc, ans);
	}
	
	printf("%ld\n", ans);
	
	return 0;
}