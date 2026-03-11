#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>
#include<limits.h>

int main(){
	int n, m;
	int res = 1;
	scanf("%d %d\n", &n, &m);
	for(long i=2; i*i <= m && i <= m/n; i++){
		if(m%i==0) res = i;
	}
	printf("%d\n", res);
	return 0;
}


