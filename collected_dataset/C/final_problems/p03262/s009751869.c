#include<stdio.h>
#include<stdlib.h>

void gcd(int *a,int b,int max){
	int tmp=*a%b;
	int i=1;
	for(i=1;i<=tmp;i++){
		if(*a%i==0&&b%i==0){
			if(max<i)max=i;
		}
	}
	*a=max;
}

int main(){
	int n,x;
	scanf("%d%d",&n,&x);
	int moto[n],sa[n],i;
	for(i=0;i<n;i++){
		scanf("%d",&moto[i]);
		sa[0]=abs(moto[0]-x);
		if(i!=0)sa[i]=abs(moto[i]-moto[i-1]);
	}
	if(n==1){
		printf("%d\n",abs(moto[0]-x));
		return 0;
	}
	int tmp=sa[0];
	gcd(&tmp,sa[1],1);
	for(i=1;i<n;i++)gcd(&tmp,sa[i],tmp);
	printf("%d\n",tmp);
	return 0;
}