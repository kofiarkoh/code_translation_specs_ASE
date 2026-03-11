#include<stdio.h>

int main(void){
	unsigned long long int i,o,t,j,l,s,z,r=0;
	scanf("%llu%llu%llu%llu%llu%llu%llu",&i,&o,&t,&j,&l,&s,&z);
	r+=o;
	if(i > 0 && l > 0 && j > 0){
		if(i <= l && i <= j){r+=(3*i);l-=i;j-=i;i=0;}
		if(l <= i && l <= j){r+=(3*l);i-=l;j-=l;l=0;}
		if(j <= i && j <= l){r+=(3*j);i-=j;l-=j;j=0;}
	}
	r+=((i/2)*2);
	r+=((l/2)*2);
	r+=((j/2)*2);
	printf("%llu\n",r);
	return 0;
}