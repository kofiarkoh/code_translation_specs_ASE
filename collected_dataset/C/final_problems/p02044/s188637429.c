#include<stdio.h>
#define min(p,q)((p)<(q)?(p):(q))

int main(){
	int n,m;
	while(scanf("%d%d",&n,&m),n){
		int ans=0;
		for(int i=0;i<n;i++){
			int t;
			scanf("%d",&t);
			ans+=min(t,m/n);
		}
		printf("%d\n",ans);
	}
}
