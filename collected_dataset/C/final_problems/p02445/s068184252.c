#include <stdio.h>

int a[1010];
int s[1010];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int q;
	scanf("%d",&q);
	while(q--){
		//[b,e)と[t,*)をswap
		int b,e,t;
		scanf("%d%d%d",&b,&e,&t);
		for(int i=0;i<e-b;i++)s[i]=a[b+i];
		for(int i=0;i<e-b;i++)a[b+i]=a[t+i];
		for(int i=0;i<e-b;i++)a[t+i]=s[i];
	}
	for(int i=0;i<n;i++)printf("%d%c",a[i],i==n-1?10:32);
}
