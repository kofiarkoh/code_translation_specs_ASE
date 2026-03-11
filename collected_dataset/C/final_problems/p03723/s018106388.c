#include<stdio.h>

int main(){
	int ans=0;
	long long a,b,c;
	scanf("%lld%lld%lld",&a,&b,&c);
	if(a==b&&b==c)puts("-1");
	else {
		while(a%2==0&&b%2==0&&c%2==0){
			if(a==b&&b==c){
				puts("-1");
				return 0;
			}
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