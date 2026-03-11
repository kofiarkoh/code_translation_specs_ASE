#include<stdio.h>

int hn(long a,long b,long c){
	if(a%2==1||b%2==1||c%2==1)return 0;
	return 1;
}

int main(){
	int ans=0;
	long a,b,c;
	scanf("%lld%ld%ld",&a,&b,&c);
	if(a==b&&b==c)puts("-1");
	else {
		while(hn(a,b,c)){
			long aa=a/2,bb=b/2,cc=c/2;
			a=bb+cc;
			b=cc+aa;
			c=aa+bb;
			ans++;
		}
		printf("%d\n",ans);
	}
	return 0;
}