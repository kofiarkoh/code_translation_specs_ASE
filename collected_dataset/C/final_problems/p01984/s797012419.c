#include<stdio.h>
void f(long long k){
	int n=54;
	while(((1LL<<(n-1))-(n-1)-1)*81>=k)n--;
	k-=((1LL<<(n-1))-(n-1)-1)*81;
	//n桁の数でk番目
	int a=1+(k-1)/(((1LL<<(n-1))-1)*9);
	k=(k-1)%(((1LL<<(n-1))-1)*9)+1;
	int b;
	putchar(a+48);
	n--;
	for(;;){
		long long lower=(long long)a<<(n-1);
		long long upper=(((1LL<<n)-1)*9)-((9LL-a)<<(n-1));
		if(k<=lower){
			b=(k-1)/(1LL<<(n-1));
			k=(k-1)%(1LL<<(n-1))+1;
			n--;
			break;
		}
		if(k>upper){
			k-=upper;
			b=(k-1)/(1LL<<(n-1))+a+1;
			k=(k-1)%(1LL<<(n-1))+1;
			n--;
			break;
		}
		putchar(a+48);
		k-=lower;
		n--;
	}
	putchar(b+48);
	k--;
	if(a<b){int t=a;a=b;b=t;}
	for(int i=n-1;i>=0;i--)putchar((k>>i)&1?a+48:b+48);
	puts("");
}

int main(){
	long long k;
	while(scanf("%lld",&k),k)f(k);
}
