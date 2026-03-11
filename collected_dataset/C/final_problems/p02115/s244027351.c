#include<stdio.h>
#define max(p,q)((p)>(q)?(p):(q))
int a[100010];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int ans=1;
	int temp=0,pre=-1;
	for(int i=0;i<n;i++){
		if(a[i]>pre)temp++,ans=max(ans,temp),pre=a[i];
		if(a[i]<pre)temp=1,pre=a[i];
	}
	temp=0,pre=2e9;
	for(int i=0;i<n;i++){
		if(a[i]<pre)temp++,ans=max(ans,temp),pre=a[i];
		if(a[i]>pre)temp=1,pre=a[i];
	}
	printf("%d\n",ans);
}
