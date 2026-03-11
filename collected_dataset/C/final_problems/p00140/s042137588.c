#include <stdio.h>
int main(){
	char tei[15]={0,1,2,3,4,5,6,7,8,9,5,4,3,2,1},ans[1000][20];
	int n,i,to,from,j;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d %d",&from,&to);
		if(from<to){
				for(j=from;j<to;j++){
					printf("%d ",j);
				}
				printf("%d\n",to);
		}else if(to==5){
			for(j=from;j<=9;j++){
				printf("%d ",j);
			}
			puts("5");
		}else if(from>to){
			if(from>=6&&from<=9){
				for(j=from;j<=9;j++){
					printf("%d ",j);
				}
				for(j=5;j>to;j--){
					printf("%d ",j);
				}
				printf("%d\n",to);
			}else{
				for(j=from;j>to;j--){
					printf("%d ",j);
				}
				printf("%d\n",to);
			}
		}
	}
	
	return 0;
}