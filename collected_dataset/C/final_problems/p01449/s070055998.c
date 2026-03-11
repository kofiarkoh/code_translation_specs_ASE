#include <stdio.h>

int N;
int masu_raw[100011];
int masu_next[100011];
char visited[100011];

int next_tansaku(int now) {
	int next=0;
	if(visited[now])return -1; /* endless loop */
	visited[now]=1;
	if(masu_next[now]!=0) {
		next=masu_next[now];
	} else if(masu_raw[now]==0) {
		next=masu_next[now]=now;
	} else {
		next=masu_next[now]=next_tansaku(now+masu_raw[now]);
	}
	visited[now]=0;
	return next;
	
}

#define INF 100000000
int memo[100001];

int min_tansaku(int now) {
	int result=INF;
	int i;
	if(now<0)return INF;
	if(now>=N)return 0;
	if(memo[now]>0)return memo[now]-1;
	memo[now]=INF+1;
	for(i=1;i<=6;i++) {
		int nowresult=min_tansaku(masu_next[now+i])+1;
		if(nowresult<result)result=nowresult;
	}
	memo[now]=result+1;
	return result;
}

int main(void) {
	int i;
	scanf("%d",&N);
	for(i=1;i<=N;i++)scanf("%d",&masu_raw[i]);
	for(i=1;i<=N+10;i++)next_tansaku(i);
	printf("%d\n",min_tansaku(1));
	return 0;
}