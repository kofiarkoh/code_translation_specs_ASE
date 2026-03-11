#include <stdio.h>
 
int ans;
int p,q,a,n;
 
void DFS (int cnt,int count,int tp,int tq,int mul)
{//已搜索到分母,个数,分数当前和分子,分母,乘积
	if (mul>a) return;
	if (p*tq==q*tp && tq!=0)
	{//找到
		ans++;
		return;
	}
	if (count>=n) return;  //个数超
	if (p*tq<q*tp) return; //比原来大
	int x,y;
	if (tq)
	{
		x=tq+cnt*tp;
		y=cnt*tq;
	}
	else x=1,y=cnt;
	if (mul*cnt<=a)
		DFS (cnt,count+1,x,y,mul*cnt);
	if (cnt+1<=a && mul*(cnt+1)<=a) //剪枝
		DFS (cnt+1,count,tp,tq,mul);
}
 
void dfs (int p,int q,int x,int dep,int mul)
{//参考acm.hust.edu.cn/vjudge/problem/viewSource.action?id=199708
	if (p==0 && mul<=a)
	{
		ans++;
		return;
	}
	if (dep==0 || p*x>q*dep || mul*x>a) return;
	
	for (int i=x;i<=a;i++)
		if (mul*i<=a)
		{
			int nt=p*i-q;
			if (nt>=0)
				dfs (nt,q*i,i,dep-1,mul*i);  
		}
		else break;
}
 
int main ()
{
	while (scanf("%d %d %d %d",&p,&q,&a,&n), p||q||a||n)
	{
		ans=0;
		dfs (p,q,1,n,1);   //219ms
		printf("%d\n",ans);
	}
	return 0;
}


