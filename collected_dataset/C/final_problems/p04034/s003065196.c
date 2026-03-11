#include <stdio.h>

#define MAX_N 100010

int main(void) {
	int N, M, x[MAX_N],y[MAX_N];
	int i,state[MAX_N],red[MAX_N],num;
	
	scanf("%d %d", &N,&M);
	
	for (i=0;i<M;i++) 
		scanf("%d %d", &x[i],&y[i]);
	
	for (i=1;i<=N;i++)
		state[i]=1;
	
	red[1]=1;
	
	for (i=0;i<M;i++) {
		state[x[i]]--;
		state[y[i]]++;
		if (red[x[i]]==1)
			red[y[i]]=1;
		if (state[x[i]]==0) 
			red[x[i]]=0;
	}
	
	num=0;
	for (i=1;i<=N;i++) {
		if (red[i]==1)
			num++;
	}

	printf("%d\n" ,num);
	
	return 0;
}