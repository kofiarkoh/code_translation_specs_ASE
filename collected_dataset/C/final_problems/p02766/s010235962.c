#include <stdio.h>
int main(){
	int n,k,cnt=0;
	scanf("%d%d",&n,&k);
	while(n){
		cnt++;
		n/=k;
	}
	printf("%d\n",cnt);
	return 0;
}