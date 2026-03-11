#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <math.h>
#define P 200003

int main(){
	int N=0;
	if(scanf("%d",&N)==1);


	int a[N];
	for(int i=0;i<N;i++){
		a[i]=0;
		if(scanf("%d ",&a[i])==1);
	}

	// printf("N=%d\n",N );
	// // printf("%d %s\n",a+b+c,s);
	
	// for(int i=0;i<N;i++){
	// 	printf("a[%d]%d\n",i,a[i]);
	// }

	int sum=0;
	for(int i=0;i<N;i++){
		for(int j=0;j<N;j++){
			if(i<j){
				int a1=a[i]%P;
				int a2=a[j]%P;

				sum+=(a1*a2)%P;
				// printf("%d %d \n",aa,sum );
			}

		}

	}	


	// int sum=0;
	// for(int i=0;i<N;i++){
	// 	for(int j=0;j<N;j++){
	// 		if(i<j){
	// 			int aa=a[i]*a[j];
	// 			sum+=aa%P;
	// 			// printf("%d %d \n",aa,sum );
	// 		}

		// }

	// }
	printf("%d\n",sum );
}
