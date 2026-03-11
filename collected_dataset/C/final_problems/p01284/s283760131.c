#include <stdio.h>
#include <string.h>

int T;
int t_cycle[30];
int limit_times[100];

int memo[30][100];

int search(int cycle,int day) {
	int ret,challenger;
	if(day>=100)return 0;
	if(memo[cycle][day]>0)return memo[cycle][day]-1;
	/* take anhydrous caffeine */
	ret=search(1%T,day+1)+1;
	/* don't take anhydrous caffeine */
	if(t_cycle[cycle]<=limit_times[day]) {
		challenger=search((cycle+1)%T,day+1);
		if(challenger<ret)ret=challenger;
	}
	memo[cycle][day]=ret+1;
	return ret;
}

int main(void) {
	while(scanf("%d",&T)==1 && T>0) {
		int N,i;
		memset(memo,0,sizeof(memo));
		for(i=0;i<T;i++) {
			if(scanf("%d",&t_cycle[i])!=1)return 1;
		}
		if(scanf("%d",&N)!=1)return 1;
		for(i=0;i<100;i++)limit_times[i]=24;
		for(i=0;i<N;i++) {
			int D,M;
			if(scanf("%d%d",&D,&M)!=2)return 1;
			if(M<limit_times[D-1])limit_times[D-1]=M;
		}
		printf("%d\n",search(0,0));
	}
	return 0;
}