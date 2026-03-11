#include <stdio.h>
#include <stdlib.h>

int main(void){
	int x,n;
	scanf("%d%d",&x,&n);
	if(n==0){
		printf("%d",x);
		return 0;
	}
	int p[n];
	for(int i=0;i<n;i++){
		scanf("%d",&p[i]);
	}
	int seek = 0;
	for(int l=0;l<=51;l++){
			for(int i=0;i<n;i++){
				if(x-l == p[i]){
					seek++;
				}
			}
			if(seek==0){
				printf("%d",x-l);
				return 0;
			}
			seek = 0;
			for(int i=0;i<n;i++){
				if(x+l == p[i]){
					seek++;
				}
			}
			if(seek==0){
				printf("%d",x+l);
				return 0;
			}
			seek=0;
		}
}
