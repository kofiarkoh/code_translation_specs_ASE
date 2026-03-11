#include<stdio.h>
int ary[10010];
int isPrime(int n){
	int i;
	if(ary[n]==2){
		return 0;
	}
	else if(ary[n]==1){
		return 1;
	}
	if(n % 2 == 0){
		ary[n]=2;
		return 0;
	}
	for(i = 3; i * i <= n; i += 2)
		if(n % i == 0){
			ary[n]=2;
			return 0;
		}
	ary[n]=1;
	return 1;
}

int main(){
	int n;
	int i,j;
	ary[0]=2;
	ary[1]=2;
	ary[2]=1;
	int c;
	for(;scanf("%d",&n)!=EOF;){
		c=0;
		for(i=1;i<=n;i++){
			if(isPrime(i)&&isPrime(n-i+1))c++;
		}
		printf("%d\n",c);

	}



	return 0;
}