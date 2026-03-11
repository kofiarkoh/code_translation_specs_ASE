//边数爆int!!! 
#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
const int N=1e5+10;
struct apple
{
	int x,y;
}edge[N];
int k,f[N];
long long cnt;
void push_back(int x,int y)
{
	edge[++k].x=x;
	edge[k].y=y;
 } 
int find(int x)
{
	if(f[x]==x)
		return x;
	f[x]=find(f[x]);
	return f[x];
}
void unioned(int x,int y)
{
	int fx,fy;
	fx=find(x);
	fy=find(y);
	if(fx!=fy)
	{
		f[fx]=fy;
		cnt--;
	} 
}
int judge(int x,int y)
{
	int fx,fy;
	fx=find(x);
	fy=find(y);
	if(fx==fy)
		return 0;
	return 1;
}
int main()
{
	int fla,q,i,x,y,z;
	long long n,m;
	fla=1;
	k=0;
	scanf("%lld%lld%d",&n,&m,&q);
	cnt=n;
	for(i=1;i<=n;i++)
		f[i]=i;
	for(i=1;i<=q;i++)
	{
		scanf("%d%d%d",&x,&y,&z);
		x++;
		y++;
		if(z==0)
			unioned(x,y);
		else
			push_back(x,y);
	}
	//for(i=1;i<=n;i++)
	//	printf("%d ",f[i]);
	//printf("\n");
	if(m==n-1)
	{
		if(k>0)
			printf("No");
		else
			printf("Yes");
		return 0;
	}
	for(i=1;i<=k;i++)
		if(!judge(edge[i].x,edge[i].y))
			fla=0;
	//printf("%d\n",fla);
	if(m>n-cnt+cnt*(cnt-1)/2)
		fla=0;
	if(fla)
		printf("Yes");
	else
		printf("No");
	return 0;
}