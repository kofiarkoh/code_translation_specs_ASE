#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

#define INF 1000000000
 
#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

typedef struct {
	long id;
	long dis;
} DATA;

long N, K;
long a[100001];
DATA data[100001];
long di = 0;
long dis[100001] = {0};

int comp(const void *c1, const void *c2) {
	DATA d1 = *(DATA *)c1;
	DATA d2 = *(DATA *)c2;
	long t1 = d1.dis;
	long t2 = d2.dis;
	
	return t2 - t1;
}

int main() {
	long i, j;
	long ans = 0;
	long now;
	long count;
	long stay[100001];
	long idx;
	long pre;
	
	scanf("%ld %ld", &N, &K);
	for(i=1;i<=N;i++) {
		scanf("%ld", &a[i]);
	}
	
	for(i=2;i<=N;i++) {
		if(dis[i] > 0) continue;
		now = i;
		count = 0;
		idx = 0;
		while(now != 1) {
			now = a[now];
			stay[idx++] = now;
			count++;
			if(dis[now] > 0) {
				count += dis[now];
				break;
			}
		}
		data[di].id = i;
		data[di++].dis = count;
		dis[i] = count;
		for(j=0;j<idx;j++) {
			if(dis[stay[j]] > 0) continue;
			dis[stay[j]] = count - j - 1;
			data[di].id = stay[j];
			data[di++].dis = dis[stay[j]];
		}
	}
	
	qsort((void *)data, di, sizeof(DATA), comp);
	
	if(a[1] != 1) ans++;
	
	if(K >= N) {
		printf("%ld\n", ans);
		return 0;
	}
	
	for(i=0;i<di;i++) {
		now = data[i].id;
		count = 0;
		while(now != 1) {
			pre = now;
			now = a[now];
			count++;
			if(count >= K && now != 1) {
				a[pre] = 1;
				ans++;
			}
		}
	}
	
	printf("%ld\n", ans);
	
	return 0;
	
}