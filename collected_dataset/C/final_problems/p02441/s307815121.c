#include <stdio.h>

int a[1010];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int q;
	scanf("%d",&q);
	while(q--){
		int l,r,k;
		scanf("%d%d%d",&l,&r,&k);
		int ans=0;
		for(int i=l;i<r;i++)ans+=a[i]==k;
		printf("%d\n",ans);
	}
}
