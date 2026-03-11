#include <stdio.h>
#define min(p,q)((p)<(q)?(p):(q))

int a[100010];
int main(){
	int n,q;
	scanf("%d%d",&n,&q);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	while(q--){
		long s;
		scanf("%ld",&s);
		long ans=0;
		int r=0;
		long sum=0;
		for(int l=0;l<n;l++){
			while(r<n&&sum<=s)sum+=a[r++];
			ans+=r-l-(sum>s);
			sum-=a[l];
		}
		printf("%ld\n",ans);
	}
}

