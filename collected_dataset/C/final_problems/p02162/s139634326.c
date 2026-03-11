#include<stdio.h>
int main(){
	int a,b,c,d;
	scanf("%d%d%d%d",&a,&b,&c,&d);
	if(c!=-1&&d!=-1)puts(c>d?"Alice":c<d?"Bob":"Draw");
	else puts(a<b?"Alice":a>b?"Bob":"Draw");
}
