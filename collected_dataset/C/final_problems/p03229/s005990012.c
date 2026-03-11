#include <stdio.h>
#include <stdlib.h>

int com(const void *a,const void *b){
	return *(int*)a - *(int*)b;
}

int main(){
	int N;
	int i,ans=0;
	
	scanf("%d",&N);
	
	int Ai[N];
	
	for(i=0;i<N;i++){
		scanf("%d",&Ai[i]);
	}
	
	qsort(Ai,N,sizeof(int),com);
	
	for(i=N-1;i>=0;i--){
		ans+=(abs(Ai[i]-Ai[N-i-1]));
	}
	
	if(Ai[N-1]!=Ai[N-2]){
		ans--;
	}
	
	printf("%d",ans);
	
	return 0;
}