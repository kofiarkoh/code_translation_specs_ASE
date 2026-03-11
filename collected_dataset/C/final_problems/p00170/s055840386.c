#include<stdio.h>
#include<string.h>
typedef struct i{char f[21];int s,w;} i;
i g[10];
int n,t[10],f[10]={0},ans[10],max=0;

void rec(int d)
{
	int i,w=0,G=0;
	if(d==n)
	{
		for(i=0;i<n;i++)
		{
			if(w>g[t[i]].s)
				return;
			w+=g[t[i]].w;
			G+=g[t[i]].w*(i+1);
		}
		if(max<G)
		{
			max=G;
			memcpy(ans,t,sizeof(t));
		}
	}
	else
	{
		for(i=0;i<n;i++)
		{
			if(!f[i])
			{
				f[i]=1;
				t[d]=i;
				rec(d+1);
				f[i]=0;
			}
		}
	}
}

int main()
{
	int i;
	for(;;)
	{
		scanf("%d",&n);
		if(!n)
			return 0;
		for(i=0;i<n;i++)
			scanf("%s %d%d",g[i].f,&g[i].w,&g[i].s);
		rec(0);
		while(--n>=0)
			puts(g[ans[n]].f);
	}
	return 0;
}