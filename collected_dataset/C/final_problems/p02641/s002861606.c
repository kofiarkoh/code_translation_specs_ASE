#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>


int p[110];
int min = 200;
int main(){
	int X,N;
	int x[10];
	scanf("%d%d",&X,&N);
	
	int i;
	
	if(N==0){
		printf("%d\n",X);
		return 0;
	}
	
	for(i=0;i<N;i++){
		int a;
		scanf("%d",&a);
		p[a]=1;
	}
	int index=0;
	int check=0;
	for(i=X-1;i>=-10;i--){
		if(p[i]==0){
			//printf("ddddd\n");
			if(abs(X-i)<min){
				
				min = abs(X-i);
				index = i;
				check=1;
				//printf("+++++  %d %d,min:%d+++++\n",i,abs(X-i),min);
			}
			else break;
		}
		if(check)break;
	}
	check=0;
	
	for(i=X+1;i<=110;i++){
		if(p[i]==0){
			//printf("ddddd\n");
			if(abs(X-i)<min){
				min = abs(X-i);
				index = i;
				//printf("+++++  %d %d,min:%d+++++\n",i,abs(X-i),min);
				check=1;
			}
			else break;
		}
		if(check)break;
	}
	
	if(index!=0)printf("%d\n",index);
	else printf("%d\n",X);

	return 0;
}
