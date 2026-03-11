#include <stdio.h>
#include <stdlib.h>
int s,ans,ttt,a,b,c,t[1000][1000];
int main(void){
	while(scanf("%d",&s)!=EOF){
		ans=0;
		for(a=0;a<=1000 && s>=a;a++){
			for(b=0;b<=1000 && s>=a+b;b++){
				ans+=501-abs(500-(s-a-b));
			}
		}
		printf("%d\n",ans);
	}
	return 0;
}