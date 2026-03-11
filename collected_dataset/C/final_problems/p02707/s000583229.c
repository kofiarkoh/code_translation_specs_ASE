#include<stdio.h>

int main(){
	int res;
	int N;
	res=scanf("%d",&N);
	int A[N+1];
	int subordinate[N+1];
	for(int i=0;i<N+1;i++){
		A[i]=0;
		subordinate[i]=0;
	}
	for(int i=2;i<=N;i++){
		res=scanf("%d",&A[i]);
	}
	for(int i=2;i<=N;i++){
		subordinate[A[i]]+=1;
	}
	for(int i=1;i<=N;i++){
		printf("%d\n",subordinate[i]);
	}
	return 0;
}
