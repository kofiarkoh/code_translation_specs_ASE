#include <stdio.h>
#include <stdlib.h>
#define SHORT 1
#define LONG 0

long hyouka(long n,long m,long book[101],long bookcase){
	long length,count = 0,i,j;
	for(i = 0 ;i < m ;i++){
		length = 0;
		for(j = 0;;j++){
			length = length + book[count];
			if(bookcase < length){
				break;
			}
			count++;
			
		}
	}
	
	if(count < n){
		return SHORT;
	}else{
		return LONG;
		}
}


long nibutan(long n,long m,long book[101]){
	long left = 0,right = 1500000,half,a;
	while(1){
		half = (right + left) / 2;
		//printf("%ld\t",half);
		
		a = hyouka(n,m,book,half);
		
		if(a == SHORT){
			left = half;
		}else if(a == LONG){
			right = half;
		}if(right == left || right == left + 1){
			break;
		}
		//printf("%ld %ld\n",left,right);
	}
	return right;
}


int main(void)
{
	long  n,m,book[101],answer,i;
	
	while(1){
		scanf("%ld %ld",&m,&n);
		
		if(n == 0 && m == 0){
			break;
		}
		
		for(i = 0;i < n;i++){
			scanf("%ld",&book[i]);
		}
		
		answer = nibutan(n,m,book);
		
		printf("%ld\n",answer);
	}
	return 0;
}