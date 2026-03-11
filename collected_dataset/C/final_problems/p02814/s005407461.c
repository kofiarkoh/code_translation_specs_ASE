#include <stdio.h>
#define ll long long
int gcd(int p,int q){while(q){int t=p%q;p=q;q=t;}return p;}

int a[100010];
int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	for(int i=0;i<n;i++)scanf("%d",a+i);
	int p2=a[0]&-a[0];
	for(int i=0;i<n;i++){
		if((a[i]&-a[i])!=p2){
			puts("0");
			return 0;
		}
		a[i]/=p2;
	}
	
	ll L=p2/2;
	for(int i=0;i<n;i++){
		int d=gcd(L,a[i]);
		if(L/d*a[i]>m){
			puts("0");
			return 0;
		}
		L=L/d*a[i];
	}
	
	printf("%lld",(m/L+1)/2);
}