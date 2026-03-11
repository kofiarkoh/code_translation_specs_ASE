#include<stdio.h>

long A[100000];
long d[100000];
long s;
long N;
long num;

long sum(long n){
	return n*(n+1)/2;
}

void input(){
	int i;
	s = 0;
	scanf("%ld", &N);
	for( i = 0; i < N; i++ ){
		scanf("%ld", &A[i]);
		s += A[i];
	}
	num = s / sum(N);
	for( i = 0; i < N-1; i++ ){ d[i] = A[i+1] - A[i] - num; }
	d[N-1] = A[0] - A[N-1] - num;
	for( i = 0; i < N; i++ ){ d[i] *= (-1); }
	return;
}

int check(){
	int i;
	long j = 0;
	for( i = 0; i < N; i++ ){
		if( d[i] % N != 0 ){ return 0; }
		j += d[i] / N;
	}
	if( j != num ){ return 0; }
	return 1;
}

int main(){
	input();
	if( num * sum(N) != s ){ printf("NO\n"); return 0; }
	if( check() == 0 ){ printf("NO\n"); return 0; }
	printf("YES\n");
	return 0;
}