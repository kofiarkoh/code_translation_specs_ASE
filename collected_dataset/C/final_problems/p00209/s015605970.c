#include<stdio.h>
#include<string.h>
int N,M,i,j,P[105][105],Q[4][55][55];

void rot(int A[55][55],int B[55][55])
{
	int i,j;
	for(i=0;i<M;i++)
		for(j=0;j<M;j++)
			B[i][j]=A[j][M-i-1];
}

int ch(int y,int x)
{
	int i,j,k,r=15;
	for(i=0;i<M;i++)
		for(j=0;j<M;j++)
		{
			for(k=0;k<4;k++)
				if(Q[k][i][j]!=-1&&P[y+i][x+j]!=Q[k][i][j])
					r&=(~(1<<k));
			if(r==0)return r;
		}
	return r;
}

void f()
{
	int i,j,k,t,r=9999999,y,x;
	for(i=0;i<=N-M;i++)
		for(j=0;j<=N-M;j++)
		{
			if(t=ch(i,j)!=0)
			{
				for(k=1;k<16;k*=2)
				{
					for(y=0;y<M;y++)
						for(x=0;x<M;x++)
							if(t&k&&Q[k][y][x]!=-1)
							{
								r=r<y*1000+x?r:y*1000+x;
								goto e;
							}
					e:;
				}
				printf("%d %d\n",j+r%1000+1,i+r/1000+1);
				return;
			}
		}
	puts("NA");
	return;
}

int main()
{
	for(;scanf("%d%d",&N,&M),N;)
	{
		for(i=0;i<N;i++)for(j=0;j<N;j++)scanf("%d",&P[i][j]);
		for(i=0;i<M;i++)for(j=0;j<M;j++)scanf("%d",&Q[0][i][j]);
		for(i=1;i<4;i++)rot(Q[i-1],Q[i]);
		f();
	}
	return 0;
}