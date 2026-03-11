#include<stdio.h>
int main(){
	long long a=0,b=1;
	int n;
	scanf("%d",&n);
	while(n--){
		int q,x;
		scanf("%d%d",&q,&x);
		if(q==1)a*=x,b*=x;
		if(q==2)a-=x;
		if(q==3)a+=x;
	}
	printf("%lld %lld\n",a,b);
}
