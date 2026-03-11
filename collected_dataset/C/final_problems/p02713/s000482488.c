#include<stdio.h>

int gcd(int a,int b,int c){
	int d=1,min;
	
	(a<b)?(min=a):(min=b);
	(min<c)?:(min=c);
	
	for(int i=0;i<min;i++){
		if(a%(i+1)==0&&b%(i+1)==0&&c%(i+1)==0){
			d=(i+1);
		}
	}
	return d;
}

int main(void){
	int k;
	long long int sum=0;
	scanf("%d",&k);
	for(int i=0;i<k;i++){
		for(int j=0;j<k;j++){
			for(int l=0;l<k;l++){
				sum+=gcd(i+1,j+1,l+1);
			}
		}
	}
	printf("%lld",sum);
}