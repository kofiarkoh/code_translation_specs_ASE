#include<stdio.h>
#define MOD 1000000007
long powll(long a, int x){
	if(x==0){
      return 1;
    }
	long b=powll(a,x/2);
	if(x%2==0){
		b=b*b%MOD;
	}else{
		b=b*b%MOD*a%MOD;
	}
	return b;
}
long rec(long a){
	return powll(a,MOD-2);
}
long fact(long a){
	long b=1;
	while(a>0){
		b=b*a%MOD;
		a--;
	}
	return b;
}
long fra_fac(int a,int b){
	return rec(fact(a))*rec(fact(b))%MOD*fact(a+b)%MOD;
}
int main(void){
	int r1,r2,c1,c2;
	scanf("%d %d %d %d",&r1,&c1,&r2,&c2);
	long ans=fra_fac(c2+1,r2+1)+fra_fac(c1,r1)-fra_fac(c2+1,r1)-fra_fac(c1,r2+1)+MOD*2;
    printf("%ld\n",ans%MOD);
    return 0;
}