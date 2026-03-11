#include<stdio.h>
#define X 1000000007

int main(void){
	unsigned long long int ans=1;
	int t[131072],a[131072],i,n;
	scanf("%d",&n);
	for(i=1;i<=n;i++){scanf("%d",&t[i]);}
	for(i=1;i<=n;i++){scanf("%d",&a[i]);}
	for(i=1;i<=n;i++){
		if(i==1){
			if(a[2] < a[1] && a[1] != t[1]){printf("0\n");return 0;}
		}
		else if(i==n){
			if(t[n-1] < t[n] && t[n] != a[n]){printf("0\n");return 0;}
		}
		else if(t[i-1] < t[i] && a[i] > a[i+1]){
			if(t[i] != a[i]){printf("0\n");return 0;}
		}
		else if(t[i-1] == t[i] && a[i] == a[i+1]){
			if(t[i] > a[i]){ans = ans * a[i] % X;}
			else{ans = ans * t[i] % X;}
		}
	}
	printf("%llu\n",ans);
	return 0;
}