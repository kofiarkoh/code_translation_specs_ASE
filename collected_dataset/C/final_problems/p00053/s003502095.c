#include<stdio.h>
int prime[10010]={0};
int c;
int isPrime(int n);
int main(){
	int i,num,sum=0;
	prime[0]=2;
	prime[1]=3;
	prime[2]=5;
	prime[3]=7;
	c=4;
	for(i=8;c<10001;i++){
		if(isPrime(i)){
			prime[c]=i;
			c++;
		}
	}
	for(;scanf("%d",&num)&&num;sum=0){
		for(i=0;i<num;i++){
			sum+=prime[i];
		}
		printf("%d\n",sum);
	}
	return 0;
}
int isPrime(int n){
	int i;
	for(i=0;i<c && prime[i]<n;i++){
		if(n==prime[i]){
			return 1;
		}
		if(n%prime[i]==0){
			return 0;
		}
	}
	for(i=3;i<n/2;i++){
		if(n%i==0)return 0;
	}
	return 1;
}