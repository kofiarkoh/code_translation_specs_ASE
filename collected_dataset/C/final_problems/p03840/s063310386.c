#include<stdio.h>

int main(void){
	unsigned long long int i,o,t,j,l,s,z,r=0;
	scanf("%llu%llu%llu%llu%llu%llu%llu",&i,&o,&t,&j,&l,&s,&z);
	r+=o;
	r+=((i/2)*2);
	r+=((l/2)*2);
	r+=((j/2)*2);
	if(i && l && j){r++;}
	printf("%llu\n",r);
	return 0;
}