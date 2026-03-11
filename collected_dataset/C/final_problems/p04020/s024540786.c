#include<stdio.h>

int main(){
		int n,i;
		long long a[100000],ans=0;
		scanf("%d",&n);
		for(i=0;i<n;i++)scanf("%lld",&a[i]);
		for(i=1;i<n;i++){
				ans+=(a[i-1]+a[i])/2;
				if(a[i]>a[i-1])a[i]=(a[i-1]+a[i])%2;
		}
		printf("%lld\n",ans);
		return 0;
}
