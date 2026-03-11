/*
 * 3027_OOLLOLL.c
 *
 *  Created on: 2019/05/19
 *      Author: nagashima
 */

#include <stdio.h>
#include <math.h>

int main(){
	int N;
	int i=0;

	scanf("%d",&N);

	for(;N>=pow(2,i)-1;i++);


	printf("%d\n",i-1);
	return 0;
}

