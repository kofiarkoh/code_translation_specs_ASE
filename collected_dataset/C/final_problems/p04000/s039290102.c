#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <math.h>

#define P 1000000007

int comp(const void *a, const void *b){return *(int*)a-*(int*)b;}
int comp2(const void *a, const void *b){
	return (*(long*)a>*(long*)b)-(*(long*)a<*(long*)b);
}

int s[900000][2], S=0;
int ans[10];
int main(void){
	int h, w, n, i, j;
	scanf("%d%d%d", &h, &w, &n);
	for(i=0; i<n; i++){
		int a, b;
		scanf("%d%d", &a, &b);
		for(j=0; j<9; j++){
			int y=a-j/3, x=b-j%3;
			if(0<y && 0<x && y<=h-2 && x<=w-2){
				s[S][0]=y;
				s[S][1]=x;
				S++;
			}
		}
	}
	qsort(s, S, sizeof(int)*2, comp2);
	long ans0=(long)(h-2)*(w-2);
	int now=1;
	for(i=1; i<S; i++){
		if(s[i][0]==s[i-1][0] && s[i][1]==s[i-1][1]){
			now++;
		}else{
			ans[now]++; ans0--;
			now=1;
		}
	}
	if(S>0){
		ans[now]++; ans0--;
	}
	printf("%ld\n", ans0);
	for(i=1; i<=9; i++){
		printf("%d\n", ans[i]);
	}
}
