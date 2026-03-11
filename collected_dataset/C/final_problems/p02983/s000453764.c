#include<stdio.h>
#include<string.h>
#include<stdlib.h>
 
#include <math.h>
 
 
 
 
 
int main(){
 
    long long int L, R, sum, i, j, tmp, tmp_i, tmp_j, min;
	
 
	scanf("%lld %lld", &L, &R);
 
	L = L % 2019;
	R = R % 2019;
   
  
	min = L*R % 2019;
  
    if(L>=R)
      R+=2019;
 
 
	for (i = L; i<R; i++) {
		for (j = i + 1; j <= R; j++){
			tmp_i = i % 2019;
			tmp_j = j % 2019;
			tmp = tmp_i*tmp_j % 2019;
			if (min>tmp)
				min = tmp;
		
		
		}
	}
 
 
	printf("%lld\n", min);
 
  return 0;
}