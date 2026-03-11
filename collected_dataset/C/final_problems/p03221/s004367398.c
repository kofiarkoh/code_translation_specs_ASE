#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <math.h>

#define P 1000000007

int comp(const void *a, const void *b){return *(int*)a-*(int*)b;}
int compw(const void *a, const void *b){return (*(int*)a>*(int*)b)-(*(int*)a<*(int*)b);}
int compu(const void *a, const void *b){
	if(*((int*)a+1)==*((int*)b+1)){
		return *((int*)a+2)-*((int*)b+2);
	}else{
		return *((int*)a+1)-*((int*)b+1);
	}
}

uint32_t nextpint(void){
	uint_fast32_t x=0;
	while(1){
		uint_fast8_t c=getchar();
		if('0'<=c && c<='9'){
			x=x*10+c-'0';
		}else{
			break;
		}
	}
	return x;
}

int data[100000][4];
int main(void){
	int n = nextpint();
	int m = nextpint();
	for(int i=0; i<m; i++){
		data[i][0] = i;
		data[i][1] = nextpint();
		data[i][2] = nextpint();
		data[i][3] = 0;
	}
	qsort(data, m, sizeof(int)*4, compu);
	int a = 0;
	int b = 0;
	for(int i=0; i<m; i++){
		if(data[i][1] != a){
			a = data[i][1];
			b = 0;
		}
		data[i][3] = ++b;
	}
	qsort(data, m, sizeof(int)*4, comp);
	for(int i=0; i<m; i++){
		printf("%06d%06d\n", data[i][1], data[i][3]);
	}
}
