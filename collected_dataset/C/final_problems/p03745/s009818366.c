#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#define ll long long int
int main(void){
	int n;
	scanf("%d",&n);
	ll a[n];
	for(int i=0;i<n;i++)scanf("%lld",&a[i]);
	int ans=1;
	bool big=false,small=false,undefine=true;
	for(int i=0;i<=n-2;i++){
		if(a[i+1]>a[i]){
			if(undefine){
				big=true;
				undefine=false;
			}
			if(small){
				small=false;
				undefine=true;
				ans++;
			}
		}
		if(a[i+1]<a[i]){
			if(undefine){
				small=true;
				undefine=false;
			}
			if(big){
				big=false;
				undefine=true;
				ans++;
			}
		}
	}
	printf("%d",ans);
	return 0;
}