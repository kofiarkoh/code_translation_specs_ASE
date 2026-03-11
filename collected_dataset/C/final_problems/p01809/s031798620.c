#include <stdio.h>
int in(void){
	int i;scanf("%d",&i);
	return i;
}
long long llin(void){
	long long i;scanf("%lld",&i);
	return i;
}
void print(int a){
	printf("%d\n",a);
}
void llprint(long long a){
	printf("%lld\n",a);
}
void print2(int a,int b){
	printf("%d %d\n",a,b);
}
long long max(long long a,long long b){
	return a>b?a:b;
}
long long min(long long a,long long b){
	return a<b?a:b;
}
int gcd(int a,int b){
	if(a%b==0){
		return b;
	}
	return gcd(b,a%b);
}
int main(void){
	int p=in(),q=in();
	int g=gcd(p,q);
	int i,j;
	p/=g;
	q/=g;
	for(i=2;i<q;i++){
		for(j=1;j<=q;j*=i){
			if(j==q){
				q=i;
				break;
			}
		}
	}
	print(q);
	return 0;
}