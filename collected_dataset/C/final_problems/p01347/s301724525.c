#include<stdio.h>

int main(){
	int i,j,N,M;
	scanf("%d %d",&N,&M);
	int x[M],y[M],s[M];
   	int ne[N][N],w[N][N];

	for(i=0;i<N;i++){
		for(j=0;j<N;j++){
			ne[i][j]=0;
			w[i][j]=0;
		}
	}

	for(i=0;i<M;i++){
		scanf("%d %d %d",&x[i],&y[i],&s[i]);
		ne[x[i]][y[i]]=1;
		w[x[i]][y[i]]=s[i];
	}

	int max[N],retval;
	retval=0;
	for(i=0;i<N;i++){
		max[i]=0;
	}

	for(i=0;i<N;i++){
		for(j=i+1;j<N;j++){
			if(ne[i][j]==1){
				if(max[j]<max[i]+w[i][j]){
					max[j] = max[i]+w[i][j];
				}
			}
		}
	}

	for(i=N-1;i>=0;i--){
		for(j=0;j<i;j++){
			if(ne[j][i]==1){
				if(max[i]-max[j]>w[j][i]){
					retval += max[i]-max[j]-w[j][i];
				}
			}
		}
	}
	printf("%d\n",retval);

	return 0;
}