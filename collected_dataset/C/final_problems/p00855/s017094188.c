#include <stdio.h>
#define MAX 1299709

int isPrime[MAX+1];
int k;

void solve(){
	if(isPrime[k]){
		printf("0\n");
		return;
	}
	int smaller,larger,i;
	for(i=k;i>0;i--){
		if(isPrime[i]){
			smaller=i;
			break;
		}
	}
	for(i=k;i<MAX;i++){
		if(isPrime[i]){
			larger=i;
			break;
		}
	}
	printf("%d\n",larger-smaller);
	return;
}

int main(void){
	int i,j;
	for(i=2;i<MAX;i++){
		isPrime[i]=1;
	}
	for(i=2;i<MAX;i++){
		if(isPrime[i]){
			for(j=2;i*j<MAX;j++){
				isPrime[i*j]=0;
			}
		}
	}
	while(scanf("%d",&k),k){
		solve();
	}

	return 0;
}