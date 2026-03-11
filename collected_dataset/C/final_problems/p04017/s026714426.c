#include<stdio.h>
#include<stdlib.h>
int x[100005]={},dp[100005][20],lg[20]={1}; //倍增
int main()
{
	int n,m,q,a,b,temp,ans,d,last,k;
	scanf("%d",&n); last=n;
	for(int i=1;i<=n;i++) scanf("%d",&x[i]);
	scanf("%d %d",&m,&q);
	for(int i=1;i<20;i++) lg[i]=lg[i-1]*2; //准备好2的幂
	for(int i=n;dp[1][0]==0;i--){ //预处理走一步的情况
		d=x[i]-m; //极限距离
		if(x[last]<d) continue; //本次结果和上一次一样 没必要搜索
		for(int j=last;j>=1&&x[j]>=d;j--,last--)
			dp[j][0]=i;
	}
	for(int j=1;j<=16;j++) //一次走65536步是可能的 走2^17是不可能的
		for(int i=1;i<=n;i++)
			dp[i][j]=dp[dp[i][j-1]][j-1]; //核心算法：在i点走2^j步等效于在i点走2^(j-1)步后再走2^(j-1)步！
	while(q>0){
		q--; ans=0;
		scanf("%d %d",&a,&b);
		if(a>b){ //a到b和b到a等效
			temp=a;
			a=b;
			b=temp;
		}
		while(a<b){
			for(k=0;k<=16;k++) //大步流星
				if(dp[a][k]>=b)
					break;
			ans+=lg[k];
			a=dp[a][k];
		}
		printf("%d\n",ans);
	}
	return 0;
}