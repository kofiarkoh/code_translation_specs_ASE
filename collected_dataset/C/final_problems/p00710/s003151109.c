	#include<stdio.h>
	#include<stdlib.h>
	
	int main(){
	int n,r;
	int p,c;
	int i,j=0,k,l=0;
	
	int box[50] = {0};
	int pre[50] = {0};
	
	while(1){
	scanf("%d %d",&n,&r);
	
	if(n==0 && r==0){
		break;
	}
	
	for(i=0;i<n;i++){
		box[i] = n-i;
		pre[i] = box[i];
	}
	
	for(j=0;j<r;j++){
		scanf("%d %d",&p,&c);
	
		for(i=0;i<c;i++){
			box[i] = pre[p+i-1];
		}
		
		for(;i<c+p-1;i++){
			box[i] = pre[i-c];
		}
		
		for(i=0;i<n;i++){
			pre[i] = box[i];
		}
		
	}
	
	printf("%d\n",box[0]);
	
	
	}
	
	return 0;
	}
