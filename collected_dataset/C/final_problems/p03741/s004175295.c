#include<stdio.h>

int main(void){
	long long n,a[131072],r,i,c,d,s;
	scanf("%lld",&n);
	for(i = 1;i <= n;i++){scanf("%lld",&a[i]);}
	s=0;c=0;
	for(i = 1;i <= n;i++){
		s+=a[i];
		if(i%2 == 1 && s <= 0){c+=(1-s);s=1;}
		if(i%2 == 0 && s >= 0){c+=(s-(-1));s=-1;}
	}
	s=0;d=0;
	for(i = 1;i <= n;i++){
		s+=a[i];
		if(i%2 == 1 && s >= 0){d+=(s-(-1));s=-1;}
		if(i%2 == 0 && s <= 0){d+=(1-s);s=1;}
	}
	if(c<d){printf("%d\n",c);}else{printf("%d\n",d);}
	return 0;
}