#include<stdio.h>
#include<stdlib.h>
#define max(a,b) ((a)>(b)?(a):(b))

int compare_int(const void *a,const void *b){
	long long A=*(long long*)a-*(long long*)b;
	if(A>0)return 1;
	if(A<0)return -1;
	return 0;
}

long long gcd(long long a,long long b){
	if(a%b==0)return b;
	return gcd(b,a%b);
}

int main(){
	int n,i;
	long long a[100000];
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%lld",&a[i]);
	if(n==2){
		printf("%lld\n",(a[0]>a[1]?a[0]:a[1]));
		return 0;
	}
	if(n==3){
		long long ab=gcd(a[0],a[1]);
		long long ac=gcd(a[2],a[1]);
		long long ad=gcd(a[2],a[0]);
		printf("%lld\n",max(ab,max(ac,ad)));
		return 0;
	}
	qsort(a,n,sizeof(long long),compare_int);
	long long aa=a[0],at,f=0,ans=1,ff,g;
	while(f==0){
		aa=a[0];g=a[0];
		for(i=1;i<n;i++){
			g=gcd(g,a[i]);
			if(gcd(aa,a[i])==1){
				at=a[i];
				f=1;
				ff=i;
				break;
			}
		}
		if(f==1)break;
		if(g==1){
			puts("1");
			return 0;
		}
		ans*=g;
		for(i=0;i<n;i++)a[i]/=g;
	}
	for(i=0;i<n;i++){
		if(ff!=i)aa=gcd(aa,a[i]);
	}
	for(i=1;i<n;i++)at=gcd(a[i],at);
	printf("%lld\n",max(aa,at)*ans);
	return 0;
}