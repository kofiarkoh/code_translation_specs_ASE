#include<stdio.h>
int main(void)
{
	int h1,h2,k1,k2,a,b,c,d,x,y;
	scanf("%d %d",&h1,&h2);
	scanf("%d %d",&k1,&k2);
	scanf("%d %d %d %d",&a,&b,&c,&d);
	x=h1*a+h2*b;
	y=k1*a+k2*b;
	if(h1>=10||k1>=10){
		x=x+h1/10*c;
		y=y+k1/10*c;
	}
	if(h2>=20||k2>=20){
		x=x+h2/20*d;
		y=y+k2/20*d;
	}
	if(x>y){
		printf("hiroshi\n");
	}
	else if(y>x){
		printf("kenjiro\n");
	}
	else {
		printf("even\n");
	}
	return 0;
}
