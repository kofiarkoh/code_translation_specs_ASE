#include <stdio.h>

int a[200010],n;
int bs(int k){
	if(a[0]>=k)return 0;
	int l=0,r=n;
	while(r-l>1){
		int m=(l+r)/2;
		if(a[m]<k)l=m;
		else r=m;
	}
	return r;
}
int bs2(int k){
	if(a[0]>k)return 0;
	int l=0,r=n;
	while(r-l>1){
		int m=(l+r)/2;
		if(a[m]<=k)l=m;
		else r=m;
	}
	return r;
}
int main(){
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int q;
	scanf("%d",&q);
	while(q--){
		int k;
		scanf("%d",&k);
		printf("%d %d\n",bs(k),bs2(k));
	}
}

