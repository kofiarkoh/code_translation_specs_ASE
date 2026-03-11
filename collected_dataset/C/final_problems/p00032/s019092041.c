#include <stdio.h>

int main(){
	int a,b,c;
	int rect=0,rhomb=0;
	
	while(scanf("%d,%d,%d",&a,&b,&c)!=EOF){
		if(a*a+b*b == c*c){
			rect++;
		}
		
		if(a==b){
			rhomb++;
		}
		
	}
	
	printf("%d\n%d\n",rect,rhomb);
	
	return 0;
}