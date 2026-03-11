#include<stdio.h>

int main(void){
	int N,i,count=0;
	char S[200000];
	scanf("%d \n %s",&N,S);
	for(i=0;i<N;i++){
		if(S[i]=='#' && S[i+1]=='.' && i<N-1){
			count++;
		}
	}
	printf("%d",count);
	return 0;
}
