//アジア地区予選2005東京大会 A Sum of Consecutive prime Numbers
#include<stdio.h>
#define MAX 10000
int n;
int primes[MAX];
int isPrime(int m){
	int i;
	if(m<=1 || m!=2&&m%2==0)return 0;
	for(i=3;i*i<=m;i+=2){
		if(m%i==0)return 0;
	}
	return 1;
}

int main(){
	int i,j;
	int num, sum,ans;
	int head,tail;
	n=0;
	for(i=2;i<=MAX;i++){
		if(isPrime(i))primes[n++]=i;
	}
	while(scanf("%d",&num)==1&&num){
		ans=0;
		head=0;
		tail=1;
		sum=2;
		while(tail<n){
			while(sum < num && tail<n-1)sum += primes[tail++];
			while(num < sum && head<tail)sum -= primes[head++];
			if(sum==num)ans++;
			if(head==tail)break;
			sum -= primes[head++];
		}
		printf("%d\n",ans);
	}
}