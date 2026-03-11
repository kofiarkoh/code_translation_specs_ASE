#include <stdio.h>
#define ll int

//next_permutation
void nexp(ll*a,int cnt){
	int l=cnt-1,r=cnt-1;
	ll t;
	while(--l>=0&&a[l]>a[l+1]);
	if(l>=0){
		while(a[l]>a[r])r--;
		t=a[l];a[l]=a[r];a[r]=t;
	}
	for(l++,r=cnt-1;l<r;l++,r--)t=a[l],a[l]=a[r],a[r]=t;
}

int a[10];
int main(){
	int n;
	scanf("%d",&n);
	if(n==1){
		puts("1");
		return 0;
	}
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int t=a[0];
	
	for(int i=0;i<n;i++)a[i]*=-1;
	nexp(a,n);
	for(int i=0;i<n;i++)a[i]*=-1;
	if(a[0]<=t)for(int i=0;i<n;i++)printf("%d%c",a[i],i==n-1?10:32);
	nexp(a,n);
	if(a[0]<=t)for(int i=0;i<n;i++)printf("%d%c",a[i],i==n-1?10:32);
	nexp(a,n);
	if(a[0]>=t)for(int i=0;i<n;i++)printf("%d%c",a[i],i==n-1?10:32);
	
	
}
