#include <stdio.h>
#define MOD 1000000007

int main(){
	int n,q;
	scanf("%d%d",&n,&q);
	int top=0;
	while(q--){
		int t,x;
		scanf("%d%d",&t,&x);
		if(t)top=(top+x)%n;
		else printf("%d\n",(top+x)%n?:n);
	}
}
