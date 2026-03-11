#include <stdio.h>

int a[1010];
int lim[1010];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=1;i<=n;i++)scanf("%d",lim+i);
	int q;
	scanf("%d",&q);
	int ans=0;
	while(q--){
		int t,x,d;
		scanf("%d%d%d",&t,&x,&d);
		if(t==1)a[x]+=d;
		else a[x]-=d;
		if(a[x]<0||a[x]>lim[x]){
			ans=x;
			break;
		}
	}
	printf("%d\n",ans);
}
