#include<stdio.h>

long long pow2(long long x){
	long long r=1;
	while(x!=0){r*=2;x--;}
	return r;
}

long long bc(long long x){
	long long r=0;
	while(x!=0){r+=(x%2);x/=2;}
	return r;
}

long long calc(long long d,long long x[]){
	long long r = 0,i,m;
	m = x[1];d/=2;
	for(i = 2;i <= 15;i++){
		if(d%2){
			if(m < x[i]){m = x[i];}
			else{r+=(m+1-x[i]);m++;}
		}
		d/=2;
	}
	return r;
}

int main(){
	long long n,k,a[16],i,m=1145141919810,x;
	scanf("%lld%lld",&n,&k);
	for(i = 1;i <= n;i++){scanf("%lld",&a[i]);}
	for(i = 1;i < pow2(n);i+=2){
		if(bc(i) != k){continue;}
		x = calc(i,a);
		if(x < m){m = x;}
	}
	printf("%lld\n",m);
	return 0;
}