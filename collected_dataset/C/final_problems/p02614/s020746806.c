#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>
#define PI 3.141592653589793
#define MOD 1000000007
//#define DEBUG 1

//qsort用、昇順
int compare_int(const void *a, const void *b)
{
	return *(int*)a-*(int*)b;
}
//qsort(data, 10, sizeof(int), compare_int)

int count, chh[8], chw[8], h, w, k, c[8][8], lim, now;

void dfs(int i, int j)
{
	if(now==lim) return;
	if(i+j==h+w){
		int black=0;
		for(int i=0; i<h; i++){
			for(int j=0; j<w; j++){
				if((chh[i]!= 1) && (chw[j]!=1) && (c[i][j]==1))
					black++;
			}
		}
		if(black == k) count++;
		now++;
		//printf("now:%d\n", now);
	}
	else if(i<h){
		chh[i] = 0;
		dfs(i+1, j);
		chh[i] = 1;
		dfs(i+1, j);
	}
	else if(j<w){
		chw[j] = 0;
		dfs(i, j+1);
		chw[j] = 1;
		dfs(i, j+1);
	}
}

int main(){
	scanf("%d%d%d", &h, &w, &k);
	char tmp;
	for(int i=0; i<h; i++){
		for(int j=0; j<w; j++){
			scanf(" %c", &tmp);
			if(tmp == '.') c[i][j] = 0;
			else c[i][j] = 1;
		}
	}

	#ifdef DEBUG
	for(int i=0; i<h; i++){
		for(int j=0; j<w; j++){
			printf("%d", c[i][j]);
		}
		printf("\n");
	}
	#endif

	lim = pow(2, h+w);
	now = 0;
	dfs(0, 0);
	
	printf("%d\n", count);
	
	
	return 0;
}

