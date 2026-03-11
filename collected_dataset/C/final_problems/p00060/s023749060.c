#include<stdio.h>

int main(void){
	int c[3],i,l,a,b,f;
	
	while(scanf(" %d %d %d",&c[0],&c[1],&c[2])!=EOF){
	f=0;
	for(i=1;i<=10;i++){
		if(c[0]+c[1]+i<=20 && i!=c[0] && i!=c[1] && i!=c[2])f++;
	}
	if(f>=4)printf("YES\n");
	else printf("NO\n");
	}
	
	return 0;
}
	