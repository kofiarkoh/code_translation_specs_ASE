#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int N,M,A[8],C[8],T[8],P[8][8],i,l,t;


void amd(int R[8])
{
	int i,j;
	for(i=0;i<N;i++)R[i]=i;
	for(i=0;i<M;i++)
		for(j=0;j<N;j++)
			if(P[i][j])
				R[j]^=R[j+1]^=R[j]^=R[j+1];
}

int dfs(int Y[8],int d,int h)
{
	int i,j,t,y[8],r=9;
	if(d==M)
	{
		for(i=t=0;i<8;i++)t+=Y[i];
		if(t!=0)return 9;
		amd(T);
		for(i=0;i<N;i++)
			if(C[i]!=T[i])
				return 9;
		return h;
	}
	for(i=0;i<(1<<N);i++)
	{
		memcpy(y,Y,sizeof(y));
		for(j=0;j<N;j++)
			if(((i>>j)&1)&&(j==0||P[d][j-1]==0)&&y[j]>0)
			{
				P[d][j]=1;
				y[j]--;
			}
		t=dfs(y,d+1,i==0?h:d);
		r=r>t?t:r;
		memset(P[d],0,sizeof(P[d]));
	}
	return r;
}

int main()
{
	
	scanf("%d%d",&N,&M);
	for(i=0;i<M;t--,P[i][t]=1,A[t]++,i++)
		scanf("%d",&t);
	amd(C);
	memset(P,0,sizeof(P));
	printf("%d\n",dfs(A,0,0)+1);
	return 0;
}