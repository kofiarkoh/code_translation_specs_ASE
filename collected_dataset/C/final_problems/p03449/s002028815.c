#include<stdio.h>
#include<math.h>

int main(void){
	
	int i,j,k;
	int N;
	scanf("%d" ,&N);
	int W[2][N];
	int re[N]; 
	int max;
	
	
	for(i = 0; i<N; i++){
		scanf("%d" ,&W[0][i]);
	}
	for(j = 0; j<N; j++){
		scanf("%d" ,&W[1][j]);
	}
	
/*	for(i = 0; i<2; i++){
		for(j = 0; j<N; j++){
			printf("%d " ,W[i][j]);
		}
		printf("\n");
	}
*/	
	for(i = 0; i<N; i++) re[i] = 0;
	
	for(i = 0; i < N; i++){
		for(j = 0; j<2; j++){
			for(k = 0; k<N; k++){
				
				if(i ==k){
					re[i] += (W[0][k] + W[1][k]);
					j++;
					//printf("a");
				}else{
					re[i] += W[j][k];
				}
				//printf("%d ",re[i]);
			}
			//printf("\n");
		}
		
	}
	
	max = re[0];
	for(i = 1; i<N; i++){
		if(re[i] > max) max = re[i];
	}
	
	printf("%d\n" ,max);
	
}
	

