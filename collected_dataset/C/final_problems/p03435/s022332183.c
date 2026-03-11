#include <stdio.h>
#include <math.h>      

#define mod 1000000007 

int main(void) {
    //code goes 
	int c11,c12,c13,c21,c22,c23,c31,c32,c33; 
	scanf("%d %d %d %d %d %d %d %d %d",&c11,&c12,&c13,&c21,&c22,&c23,&c31,&c32,&c33); 
	if(c11+c22==c21+c12 && c21+c32==c22+c31 && c12+c23==c22+c13 && c33+c22==c23+c32) 
	    printf("Yes"); 
	else      
	    printf("No");
	return 0;
}

