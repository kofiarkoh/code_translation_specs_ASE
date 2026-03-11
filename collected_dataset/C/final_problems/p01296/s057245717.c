#include<stdio.h>
#include<string.h>
int N,X[2000],Y[2000],D[2000],f[2000][6],i,j,k;
int A[2][6][2]={{{-1,0},{0,-1},{1,-1},{2,0},{1,1},{0,1}}
,{{-1,0},{-1,1},{0,2},{1,-1},{1,0},{0,-1}}};

int main()
{
	for(;scanf("%d",&N),N;)
	{
		memset(f,-1,sizeof(f));
		for(i=0;i<N;i++)
		{
			scanf("%d%d %c\n",&X[i],&Y[i],&D[i]);
			D[i]-='x';
			for(j=0;j<i;j++)
				for(k=0;k<6;k++)
				{
					if(X[j]==X[i]+A[D[i]][k][0]&&Y[j]==Y[i]+A[D[i]][k][1])f[i][k]=j;
					else if((X[j]+D[j]?0:1)==X[i]+A[D[i]][k][0]&&(Y[j]+D[j]?1:0)==Y[i]+A[D[i]][k][1])f[i][k]=j;
				}
		}
		for(i=0;i<N;i++)
		{
			if(!D[i]&&((f[i][1]==f[i][2]&&f[i][1]!=-1)||(f[i][4]==f[i][5]&&f[i][4]!=-1)))break;
			else if(D[i]&&((f[i][0]==f[i][1]&&f[i][0]!=-1)||(f[i][3]==f[i][4]&&f[i][4]!=-1)))break;
		}
		puts(i==N?"Yes":"No");
	}
	return 0;
}