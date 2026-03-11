#include <stdio.h>

int a[1010];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int q;
	scanf("%d",&q);
	while(q--){
		int l,r;
		scanf("%d%d",&l,&r);
		for(int i=0;l+i<r-1-i;i++){
			int t=a[l+i];
			a[l+i]=a[r-1-i];
			a[r-1-i]=t;
		}
	}
	for(int i=0;i<n;i++)printf("%d%c",a[i],i==n-1?10:32);
}
