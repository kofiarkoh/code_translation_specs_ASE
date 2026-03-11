#include<stdio.h>
#include<string.h>
int N,i,j,k,G[30][30],f;
char S[501][11],*s,*t;
int main()
{
	for(;scanf("%d\n",&N),N;)
	{
		f=0;
		memset(G,0,sizeof(G));
		for(i=0;i<N;i++)
			gets(S[i]);
		for(i=1;i<N;i++)
		{
			s=S[i-1],t=S[i];
			for(j=0;s[j]&&s[j]==t[j];j++);
			if(s[j]&&!t[j])
				f=1;
			else if(s[j]&&t[j]&&s[j]!=t[j])
				G[s[j]-'a'][t[j]-'a']=1;
		}
		for(k=0;k<26;k++)
			for(i=0;i<26;i++)
				for(j=0;j<26;j++)
					G[i][j]|=G[i][k]&&G[k][j];
		for(i=0;i<26;i++)
			f|=G[i][i];
		puts(f?"no":"yes");
	}
	return 0;
}