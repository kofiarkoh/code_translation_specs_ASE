#include <stdio.h>

int main(){
	int A[4], B[4];
	int a, b;
	int hit, blow;

	while( scanf("%d%d%d%d",&A[0],&A[1],&A[2],&A[3]) != EOF ){
		scanf("%d%d%d%d",&B[0],&B[1],&B[2],&B[3]);

		hit=blow=0;

		for( a=0; a<4; a++ ){
			for( b=0; b<4; b++ ){
				if( A[a] == B[b] ){
					(a==b)? hit++ : blow++;
				}
			}
		}

		printf("%d %d\n", hit, blow);
	}

	return 0;
}
	