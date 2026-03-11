#include<stdio.h>

int main(void){
	int n;
	char s[4000];
	long long int r=0,g=0,b=0,sum,dis=0;
	scanf("%d",&n);
	scanf("%s",s);
	for(int i=0;i<n;i++){
		if(s[i]=='R'){
			r+=1;
		}else if(s[i]=='G'){
			g+=1;
		}else{
			b+=1;
		}
	}
	sum=r*g*b;
	for(int i=0;i<n-2;i++){
		for(int j=i+1;j<n-1;j++){
			if(s[i]!=s[j]){
				if(2*j-i<n&&s[2*j-i]!=s[j]&&s[2*j-i]!=s[i]){
					dis+=1;
				}
			}
		}
	}
	printf("%lld",sum-dis);
}