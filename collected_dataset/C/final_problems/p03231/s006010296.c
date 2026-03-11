#include <stdio.h>
int main(void){
	int n,m;
	char s[100001],t[100001];
	scanf("%d %d",&n,&m);
	scanf("%s",&s);
	scanf("%s",&t);
	int a,b,c;
	if(n>m){
		a=n;
		b=m;
	}else{
		a=m;
		b=n;
	}
	while(1==1){
		c=a%b;
		if(c==0){
			break;
		}
		a=b;
		b=c;
	}
	long long int ans;
	ans = 1;
	ans = ans*n/b*m;

	int i;
	for(i=0;i<b;i++){
		if(s[(n/b)*i]!=t[(m/b)*i]){
			ans = -1;
			break;
		}
	}
	printf("%lld\n",ans);
	return 0;
}