#include <stdio.h>
#include <stdlib.h>

long long int inf=1000000000000000000;

long long int Change(int x,int K,long long int **C,int N){
	long long int ans=inf;
	long long int new_ans; 
	int i;
	if (C[x-1][K-1]!=inf){
		long long int ans=C[x-1][K-1];
		return ans;
	}
	else {
		for (i=0;i<N;i++){
			if (C[x-1][i]!=inf&&C[x-1][i]!=-1){
				long long int t=C[x-1][i];
				C[x-1][i]=C[i][x-1]=-1;
				new_ans=t+Change(i,K,C,N);
				if (new_ans<ans){
					ans=new_ans;
				}
				C[x-1][i]=C[i][x-1]=t;
			}
		}
		return ans;
	}
}

int main(){
	int N;
	scanf("%d",&N);
	long long int **C=malloc(N*sizeof(long long int*));
	int a,b;
	long long int c;
	int i,j,k;
	for (i=0;i<N;i++){
		C[i]=malloc(N*sizeof(long long int));
		for (j=0;j<N;j++){
			C[i][j]=inf;
		}
	}
	for (i=0;i<N-1;i++){
		scanf("%d %d",&a,&b);
		scanf("%lld",&c);
		C[a-1][b-1]=C[b-1][a-1]=c;
	}
	for (i=0;i<N;i++){
		for (j=0;j<N;j++){
			for (k=0;k<N;k++){
				if (C[i][j]>C[i][k]+C[k][j]){
					C[i][j]=C[i][k]+C[k][j];
				}
			}
		}
	}

	int Q,K;
	int x,y;
	scanf("%d %d",&Q,&K);

	for (i=0;i<Q;i++){
		scanf("%d %d",&x,&y);
		if (C[x-1][K-1]==inf){
			C[x-1][K-1]=C[K-1][x-1]=Change(x,K,C,N);
		}
		if (C[y-1][K-1]==inf){
			C[y-1][K-1]=C[K-1][y-1]=Change(y,K,C,N);
		}
		long long int ans=C[x-1][K-1]+C[y-1][K-1];
		printf("%lld\n",ans);
	}
	return 0;
}