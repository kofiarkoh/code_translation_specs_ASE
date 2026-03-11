#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//????????¢??°
int compare(char *a, char *b){
	return strcmp(a, b);
}

//??????
void sort_sub(char **origin, int left, int right, char **tmp){
	if(right - left > 1){
		int i, j, k, half = (left + right) / 2;
		sort_sub(origin, left, half, tmp);
		sort_sub(origin, half, right, tmp);
		for(i = left; i < right; i++){
			tmp[i] = origin[i];
		}
		for(i = left, j = left, k = half; i < right; i++){
			if(k == right){
				origin[i] = tmp[j];
				j++;
			}
			else if(compare(tmp[j], tmp[k]) <= 0 && j < half){
				origin[i] = tmp[j];
				j++;
			}
			else{
				origin[i] = tmp[k];
				k++;
			}
		}
	}
}

void sort(char **origin, int N){
	char **tmp = (char **)malloc(sizeof(char *) * N);
	sort_sub(origin, 0, N, tmp);
	free(tmp);
}

int main(){
	int N, L, i, j, k, l;
	scanf("%d%d", &N, &L);
	char **s = (char **)malloc(sizeof(char *) * N);
	int **is_reverse = (int **)malloc(sizeof(int *) * N);
	int *is_used = (int *)malloc(sizeof(int) * N);
	int *go = (int *)malloc(sizeof(int) * N);
	int *back = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		s[i] = (char *)malloc(sizeof(char) * (L + 1));
		scanf("%s", s[i]);
		is_reverse[i] = (int *)malloc(sizeof(int) * N);
		for(j = 0; j < N; j++){
			is_reverse[i][j] = 1;
		}
		is_used[i] = 0;
	}
	sort(s, N);
	for(i = 0; i < N; i++){
		for(j = 0; j < N; j++){
			for(k = 0; k < L; k++){
				if(s[i][k] != s[j][L - 1 - k]){
					is_reverse[i][j] = 0;
					is_reverse[j][i] = 0;
					break;
				}
			}
		}
	}
	l = 0;
	for(i = 0; i < N - 1; i++){
		if(is_used[i] == 0){
			for(j = i + 1; j < N; j++){
				if(is_used[j] == 0 && is_reverse[i][j] == 1){
					is_used[i] = 1;
					is_used[j] = 1;
					go[l] = i;
					back[l] = j;
					l++;
					break;
				}
			}
		}
	}
	for(i = 0; i < l; i++){
		for(k = 0; k < L; k++){
			printf("%c", s[go[i]][k]);
		}
	}
	for(i = 0; i < N; i++){
		if(is_reverse[i][i] == 1 && is_used[i] == 0){
			for(k = 0; k < L; k++){
				printf("%c", s[i][k]);
			}
			break;
		}
	}
	for(i = l - 1; i >= 0; i--){
		for(k = 0; k < L; k++){
			printf("%c", s[back[i]][k]);
		}
	}
	printf("\n");
	return 0;
}