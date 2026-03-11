#include<stdio.h>
#include<string.h>
#define M 100000
#define I(x) memset(x,0,sizeof(x))
int m[21][21],bn[21],i,j,n,a,b,t,f,s[M][21],p[M],c[M],r,l,res;
int main()
{
	for(;scanf("%d",&n),n;)
	{
		I(m);I(bn),I(s);I(s);I(p);I(c);
		for(i=0;i<n-1;i++)
		{
			scanf("%d%d%d",&a,&b,&t);
			a--,b--;
			bn[a]++;bn[b]++;
			m[a][b]=m[b][a]=t;
		}
		r=l=0;res=M;
		for(s[l++][0]=1;r<=l&&l<M;r++)
		{
			for(i=0;i<=n&&s[r][i];i++);
			if(i==n){res=res>c[r]?c[r]:res;break;};
			for(i=0;i<n;i++)
				if(m[p[r]][i]!=0&&s[r][i]<=bn[i])
				{
					memcpy(s[l],s[r],sizeof(int[21]));
					s[l][i]++;
					p[l]=bn[i]==1?p[r]:i;
					c[l]=bn[i]==1?c[r]:c[r]+m[p[r]][i];
					l++;
				}
		}
		printf("%d\n",res);
	}
	return 0;
}