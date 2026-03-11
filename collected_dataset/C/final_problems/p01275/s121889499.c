#include<stdio.h>
#include<string.h>
#include<math.h>

int N,i,d[10],f[10];
char P[15],A[15];

int dis(int s,int e,int d)
{
	int i,r;
	for(i=s,r=0;i!=e;i=(i+d+10)%10)r++;
	return r*d;
}

int dfs(int p)
{
	int i,j,r=9999,t1,t2;
	if(p==N)
	{
		for(i=-10;i<10;i++)
		{
			t1=1;
			for(j=0;j<N;j++)
				if(d[j]!=i&&(j==0||d[j]!=d[j-1]))
					t1++;
			r=r>t1?t1:r;
		}
		return r;
	}
	d[p]=dis(P[p]-'0',A[p]-'0',1);
	t1=dfs(p+1);
	d[p]=dis(P[p]-'0',A[p]-'0',-1);;
	t2=dfs(p+1);
	return t1>t2?t2:t1;
}

int main()
{
	for(;scanf("%d\n",&N),N;)
	{
		scanf("%s%s",P,A);
		printf("%d\n",dfs(0));
	}
	return 0;
}