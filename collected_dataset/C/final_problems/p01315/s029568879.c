#include <stdio.h>
#include <string.h>

typedef struct {
	char L[21];
	double efficient;
} CROP;

CROP crop[50];

int main(void)
{
	int N;
	int P, A, B, C, D, E, F, S, M;
	int i, j;
	int days, income;
	CROP t;
	
	while (1){
		scanf("%d", &N);
		if (N == 0){
			break;
		}
		
		
		for (i = 0; i < N; i++){
			scanf("%s", crop[i].L);
			
			scanf("%d%d%d%d%d%d%d%d%d", &P, &A, &B, &C, &D, &E, &F, &S, &M);
			
			days = A + B + C + (D + E) * M;
			income = M * F * S - P;
			
			crop[i].efficient = (double)income / days;
		}
		
		for (i = 0; i < N; i++){
			for (j = 0; j < N - 1; j++){
				if (crop[j].efficient < crop[j + 1].efficient){
					t = crop[j];
					crop[j] = crop[j + 1];
					crop[j + 1] = t;
				}
				else if (crop[j].efficient == crop[j + 1].efficient &&
				         strcmp(crop[j].L, crop[j + 1].L) > 0){
					t = crop[j];
					crop[j] = crop[j + 1];
					crop[j + 1] = t;
				}
			}
		}
		
		for (i = 0; i < N; i++){
			printf("%s\n", crop[i].L);
		}
		printf("#\n");
	}
	
	return (0);
}