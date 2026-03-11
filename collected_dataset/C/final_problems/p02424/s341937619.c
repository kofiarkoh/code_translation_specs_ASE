#include <stdio.h>

void p2(unsigned int n){
	for(int i=31;i>=0;i--)printf("%u",(n>>i)&1);
	puts("");
}
int main(){
	unsigned int n,m;
	scanf("%u%u",&n,&m);
	p2(n&m);
	p2(n|m);
	p2(n^m);
}
