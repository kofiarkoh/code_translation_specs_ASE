#include <stdio.h>
#include <stdlib.h>

int n,s,count;

void search_program( int sum, int deapth, int q ){
	int i;

	if( deapth > n ){
		if( sum == s ) count++;
		return;
	}

	for( i = q ; i <= 9 ; i ++ ){
		if( sum + i > s ) return;
		search_program( sum+i, deapth++, i+1 );
	}
}

int main(){
	while( scanf("%d %d",&n,&s) , n && s ){
	count = 0;
	search_program(0,0,0);
	printf("%d\n",count);
	}
	return 0;
}