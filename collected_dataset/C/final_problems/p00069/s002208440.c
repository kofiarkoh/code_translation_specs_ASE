#include <stdio.h>
#include <string.h>

int i, j;
int flag;
int n;
int m;
int atari;
int d;
char amida[11][31];
int temp[10], keep[10];

int check_amida(void){
	int k, l;
	for (k = 0; k < d; k++){
		for (l = 0; l < n - 1; l++){
			if (amida[k][l] == '1'){
				temp[l] += temp[l + 1];
				temp[l + 1] = temp[l] - temp[l + 1];
				temp[l] -= temp[l + 1];
			}
		}
	}
	if (temp[atari - 1] == m){
		return (1);
	}
	else {
		return (0);
	}
}

int ins_amida(int a, int b){
	int res;
	amida[a][b] = '1';
	res = check_amida();
	amida[a][b] = '0';
	
	return (res);
}

int main(void)
{
	while (1){
		scanf("%d", &n);
		
		if (!n){
			break;
		}
		
		for (i = 0; i < n; i++){
			temp[i] = i + 1;
			keep[i] = i + 1;
		}
		
		scanf("%d", &m);
		scanf("%d", &atari);
		scanf("%d", &d);
		
		for (i = 0; i < d; i++){
			scanf("%s", amida[i]);
			getchar();
		}
		
		flag = 0;
		
		if (check_amida() == 1){
			printf("0\n");
			flag = 1;
		}
		if (flag == 0){
			for (i = 0; i < d; i++){
				for (j = 0; j < n - 1; j++){
					memcpy(temp, keep, sizeof(keep));
					if (amida[i][j] == '1'){
						continue;
					}
					if (j > 0){
						if (amida[i][j - 1] == '1'){
							continue;
						}
					}
					if (j != n - 2){
						if (amida[i][j + 1] == '1'){
							continue;
						}
					}
					if (ins_amida(i, j) == 1){
						printf("%d %d\n", i + 1, j + 1);
						flag = 1;
						break;
					}
				}
				if (flag == 1){
					break;
				}
			}
		}
		if (flag == 0){
			printf("1\n");
		}
	}
	return (0);
}