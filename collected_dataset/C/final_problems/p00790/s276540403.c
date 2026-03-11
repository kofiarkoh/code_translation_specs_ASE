/*
 * main.c
 *
 *  Created on: 2016/08/28
 *      Author: Wakaki
 */
#include<stdio.h>

int main(){

	int i,n;
	int top,north,south,east,west,bottom,temp;
	char direction[8];

	while(1){
		top=1;
		north=2;
		west=3;
		east=4;
		south=5;
		bottom=6;

		scanf("%d",&n);
		if(n==0){
			break;
		}

		for(i=0;i<n;i++){
			scanf("%s",direction);

		    if(direction[0]=='n'){
		    	temp=top;
		    	top=south;
		    	south=bottom;
		    	bottom=north;
		    	north=temp;
		    }
		    else if(direction[0]=='s'){
		    	temp=top;
		    	top=north;
		    	north=bottom;
		    	bottom=south;
		    	south=temp;
		    }
		    else if(direction[0]=='w'){
		    	temp=top;
		    	top=east;
		    	east=bottom;
		    	bottom=west;
		    	west=temp;
		    }
		    else if(direction[0]=='e'){
		    	temp=top;
		    	top=west;
		    	west=bottom;
		    	bottom=east;
		    	east=temp;
		    }
		}
		printf("%d\n",top);
	}

	return(0);
}