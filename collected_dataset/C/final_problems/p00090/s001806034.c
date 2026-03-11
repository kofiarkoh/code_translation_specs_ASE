// AOJ0090 Overlaps of Seals

#include <stdio.h>
#include <string.h>

#define MAX_N	(200)


int main(void)
{
    int n;
	double circle[MAX_N][2];
    double d;
    int flag[MAX_N][MAX_N];
    int overlap[MAX_N][MAX_N];
    int max_overlap;
	int i, j, k;
	int num[MAX_N], max[MAX_N], min[MAX_N];
    int cont_flag;
	
	while (1){
		scanf("%d", &n);
		if (n == 0){
			break;
		}
		
		// üÍ
		for (i = 0; i < n; i++){
			scanf("%lf,%lf", &circle[i][0], &circle[i][1]);
		}
		
		// dÈèe[uì¬
		memset(flag, 0, sizeof(flag));
		for (i = 0; i < n; i++){
			for (j = i; j < n; j++){
				if (i == j){
					flag[i][j] = 1;
				}
				else {
					d = (circle[i][0] - circle[j][0]) * (circle[i][0] - circle[j][0]) +
					    (circle[i][1] - circle[j][1]) * (circle[i][1] - circle[j][1]);
					if (d <= 4.0){
						flag[i][j] = flag[j][i] = 1;
					}
				}
			}
		}
        
                    // [Á±Ìí
        max_overlap = 1;
        do {
			for (i = 0; i < n; i++){
                num[i] = 0;
				for (j = 0; j < n; j++){
					num[i] += flag[i][j];
				}
   			}

            cont_flag = 0;
			for (i = 0; i < n; i++){
                if (num[i] == 2){
                    max_overlap = 2;
                }
				if (num[i] == 1 || num[i] == 2){
					for (j = 0; j < n; j++){
						flag[i][j] = flag[j][i] = 0; 
					}
                    cont_flag = 1;
				}
			}
		} while (cont_flag == 1);

        
		memset(overlap, 0, sizeof(overlap));
        for (i = 0; i < n; i++){
            if (num[i] > 0){
                for (j = 0; j < n; j++){
                    if (flag[i][j] == 1){
                        for (k = j + 1; k < n; k++){
                            if (flag[i][k] == 1){
                                if (flag[j][k] == 1){
                                    overlap[i][j]++;
                                    overlap[i][k]++;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        for (i = 0; i < n; i++){
            max[i] = 0;
            min[i] = 1000;
        }
        for (i = 0; i < n; i++){
            for (j = 0; j < n; j++){
                if (max[i] < overlap[i][j]){
                    max[i] = overlap[i][j];
                }
                if (overlap[i][j] > 0 && min[i] > overlap[i][j]){
                    min[i] = overlap[i][j];
                }
            }
//            printf("<%d %d-%d>", i, min[i], max[i]);
        }

        for (i = 0; i < n; i++){
            for (j = 0; j < n; j++){
//                printf("[%d]",overlap[i][j]);
                if (overlap[i][j] < overlap[i][i] || max[i] == min[i]){
                    if (max_overlap < overlap[i][j]){
                        max_overlap = overlap[i][j];
                    }
                }
            }
//            printf("\n");
        }
        
        printf("%d\n", max_overlap + 1);
	}
	
	return (0);
}