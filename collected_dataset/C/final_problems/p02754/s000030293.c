#include <stdio.h>
int main(void){
	int N=0;
	int A=0;
	int B=0;
	int res = 0;
	
	scanf("%d%d%d",&N,&A,&B);
	while(N > 0){
		if( N - A > 0){
			N = N - A;
			res = res + A;
		}else{
			res = res + A + (N - A);
		}
		N = N - B;
		
	}
	printf("%d\n",res);
	
	
	
	
	return 0;
}