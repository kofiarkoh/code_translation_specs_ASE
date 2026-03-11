#include<stdio.h>

#define N 100000
long A[N];
long B[N];
int n;
long count;

void init(){
	count = 0;
	return;
}

void input(){
	int i;
	scanf("%d", &n);
	for( i = 1; i <= n; i++ ){
		scanf("%ld %ld", &A[i], &B[i]);
	}
	return;
}

int push(int i){
	//if( i <= 0 ){ return 0; }
	int t = B[i] - (A[i]+count) % B[i];
	count += (t==B[i]?0:t);
	//push(i-1);
	return 0;
}

int main(){
	init();
	input();
	int i;
	for( i = n; i >= 1; i-- ){
		push(i);
	}
	//push(n);
	printf("%ld\n", count);
	return 0;
}
