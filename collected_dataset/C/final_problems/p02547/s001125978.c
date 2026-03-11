//ABC179_B
#include <stdio.h>

int main(){
	int n;
	int d1[105],d2[105];
	int flag=0;
	
	scanf("%d",&n);

	for(int i=0;i<n;i++){
		scanf("%d %d",&d1[i],&d2[i]);
	}
	
	for(int i=0;i<n-2;i++){
		if(d1[i]==d2[i] && d1[i+1]==d2[i+1] && d1[i+2]==d2[i+2]){
			flag = 1;
		}
	}
	
	if(flag == 1)	printf("Yes");
	else printf("No");
	
	return 0;
}
