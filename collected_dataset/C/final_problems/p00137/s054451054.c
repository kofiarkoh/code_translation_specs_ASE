#include<stdio.h>
#include<string.h>
int main(){
	int n,x,i,j,k;
	char s[10];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&x);
		printf("Case %d:\n",i+1);
		for(j=0;j<10;j++){
			x=x*x;
			for(k=0;k<8;k++){
				s[k]=x%10+'0';
				x=x/10;
			}
			x=0;
			for(k=5;k>=2;k--){
				x*=10;
				x+=s[k]-'0';
			}
			printf("%d\n",x);
		}
	}
	return 0;
}