#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int m,i,j,k,ans,price;

int main(void){

	while(scanf("%d",&m)*m){
		m=m/100;ans=1000000;
		for(i=0;i<=m;i+=5){
			for(j=0;j<=m-i;j+=3){
				if((m-i-j)%2==0){
					price=(i/15)*2244+(i%15)/5*850+(j/12)*1870+(j%12)/3*550+((m-i-j)/10)*1520+((m-i-j)%10)/2*390;
					if(price<ans)ans=price;
				}
			}
		}
		printf("%d\n",ans);
	}

	return 0;
}