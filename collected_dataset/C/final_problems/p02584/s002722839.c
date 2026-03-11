#include<stdio.h>
int main(void)
{
	long long a,b,c,d,e,f,g;
	scanf("%lld %lld %lld",&a,&b,&c);//a位置,b回数,c距離//
	if(a<0) a=-a;
	d=a/c;/*移動回数*/
	e=a%c;/*位置*/
	if(d<=b){
		f=d-b;
		if(f%2==0){
			printf("%lld\n",e);
		}
		else {
			printf("%lld\n",c-e);
		}
	}
	else {
		printf("%lld\n",a-(b*c));
	}
	return 0;
}