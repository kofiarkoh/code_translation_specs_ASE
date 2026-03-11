#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#define ll long long int
#define lim 100010
int MIN(int x, int y){return x<y?x:y;}
int compare_longlong(const void *a, const void *b) {long long *A = (long long *)a;long long *B = (long long *)b;if (*A > *B) return 1;if (*A < *B) return -1;return 0;}

typedef struct{
	int x,y;
	bool use;
}point;

int comp(const void *a, const void *b){return ((point*)a)->x - ((point*)b)->x;}

int main(void){
	int n;
	scanf("%d",&n);
	point red[n],blue[n];
	for(int i=0;i<n;i++)scanf("%d%d",&red[i].x,&red[i].y);
	for(int i=0;i<n;i++)scanf("%d%d",&blue[i].x,&blue[i].y);
	for(int i=0;i<n;i++)red[i].use = blue[i].use = false;
	qsort(red,n,sizeof(point),comp);
	qsort(blue,n,sizeof(point),comp);
	for(int i=0;i<n;i++){
		bool first=false;
		int index=-1;
		for(int j=0;j<n;j++){
			if(!red[j].use && red[j].y<blue[i].y && red[j].x<blue[i].x && !first){
				index=j;
				first=true;
			}
			if(!red[j].use && red[j].y<blue[i].y && red[j].x<blue[i].x && first){
				if(red[index].y<red[j].y)index=j;
			}
		}
		if(index>=0)red[index].use = true;
	}
	int ans=0;
	for(int i=0;i<n;i++){
		if(red[i].use)ans++;
	}
	printf("%d",ans);
	
	return 0;
}